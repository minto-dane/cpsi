use crate::dependency;
use crate::repository;

use cps_common::{errors::CpsiError, package::Package};

/// Install specified packages.
/// ## Arguments
/// - package_name: &[String]
///  - Variable-length arguments of packages to be installed.
///  - The specified packages will be installed.
pub fn install(package_names: &[String]) -> Result<(), CpsiError> {
    let mut install_packages: Vec<&Package> = Vec::new();

    let repository_db = repository::parquet::Repository::load()?;

    for pkg in package_names {
        let package = repository_db
            .find_package(pkg)
            .ok_or(CpsiError::PackageNotFound(pkg.clone()))?;

        install_packages.push(package);
    }

    let deps = dependency::resolve::resolve(&install_packages, &repository_db)?;

    eprintln!("---- Debug ----");
    eprintln!("total packages length: {}", deps.len());
    eprintln!(
        "all package names: {}",
        deps.iter()
            .map(|f| f.name.clone())
            .collect::<Vec<String>>()
            .join(", ")
    );

    Ok(())

    /*
     * let info = repository::find(package_names).unwrap();
     *
     * let deps = dependency::resolve(&info).unwrap();
     *
     * download::packages(&deps)?;
     *
     *
     * signature::verify(&deps)?;
     *
     * packages::extract(&deps)?;
     *
     * package::run_scripts(&deps)?;
     *
     *
     * database::register(&deps)?;
     *
     */
}
