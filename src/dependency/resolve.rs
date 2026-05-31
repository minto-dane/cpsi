use std::collections::HashSet;

use crate::repository::parquet::Repository;

use cps_common::{dependency::Dependency, errors::CpsiError, package::Package};

/// Resolve package dependencies in install order.
///
/// Dependencies are emitted before the package that requires them. Packages
/// already added to the result are de-duplicated by package name.
pub fn resolve<'a>(
    targets: &[&'a Package],
    repository: &'a Repository,
) -> Result<Vec<&'a Package>, CpsiError> {
    let mut resolver = Resolver {
        repository,
        visiting: HashSet::new(),
        visited: HashSet::new(),
        resolved: Vec::new(),
    };

    for target in targets {
        resolver.visit(target)?;
    }

    Ok(resolver.resolved)
}

/// Resolve package names against the repository, then resolve their
/// dependencies in install order.
pub fn resolve_names<'a, I, S>(
    package_names: I,
    repository: &'a Repository,
) -> Result<Vec<&'a Package>, CpsiError>
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    let mut targets = Vec::new();

    for name in package_names {
        let name = name.as_ref();
        let package = repository
            .find_package(name)
            .ok_or_else(|| CpsiError::PackageNotFound(name.to_string()))?;
        targets.push(package);
    }

    resolve(&targets, repository)
}

struct Resolver<'a> {
    repository: &'a Repository,
    visiting: HashSet<String>,
    visited: HashSet<String>,
    resolved: Vec<&'a Package>,
}

impl<'a> Resolver<'a> {
    fn visit(&mut self, package: &'a Package) -> Result<(), CpsiError> {
        if self.visited.contains(&package.name) {
            return Ok(());
        }

        if !self.visiting.insert(package.name.clone()) {
            return Err(CpsiError::DependencyCycleDetected);
        }

        for dependency in &package.dependencies {
            let dependency_package = self.resolve_dependency(dependency)?;
            self.visit(dependency_package)?;
        }

        self.visiting.remove(&package.name);
        self.visited.insert(package.name.clone());
        self.resolved.push(package);

        Ok(())
    }

    fn resolve_dependency(&self, dependency: &Dependency) -> Result<&'a Package, CpsiError> {
        if let Some(package) = self.repository.find_package(&dependency.name) {
            if dependency_is_satisfied(package, dependency) {
                return Ok(package);
            }
        }

        let candidates: Vec<_> = self
            .repository
            .packages()
            .filter(|package| package.provides.iter().any(|name| name == &dependency.name))
            .filter(|package| dependency_is_satisfied(package, dependency))
            .collect();

        match candidates.as_slice() {
            [package] => Ok(*package),
            [] => Err(CpsiError::UnsatisfiedDependency(format_dependency(
                dependency,
            ))),
            _ => {
                let mut provider_names: Vec<_> = candidates
                    .iter()
                    .map(|package| package.name.as_str())
                    .collect();
                provider_names.sort_unstable();

                Err(CpsiError::AmbiguousProvider(
                    dependency.name.clone(),
                    provider_names.join(", "),
                ))
            }
        }
    }
}

fn dependency_is_satisfied(package: &Package, dependency: &Dependency) -> bool {
    dependency
        .min_version
        .as_ref()
        .is_none_or(|min_version| &package.version >= min_version)
}

fn format_dependency(dependency: &Dependency) -> String {
    match &dependency.min_version {
        Some(version) => format!("{}>={}", dependency.name, version.to_string()),
        None => dependency.name.clone(),
    }
}
