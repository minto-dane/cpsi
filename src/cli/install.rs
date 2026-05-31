use crate::dependency;
use crate::package::Package;
use crate::repository;
use crate::util::errors::CpsiError;

/// Install specified packages.
/// ## Arguments
/// - package_name: &[String]
///  - Variable-length arguments of packages to be installed.
///  - The specified packages will be installed.
pub fn install(package_names: &[String]) -> Result<(), CpsiError> {
    let mut install_packages: Vec<&Package> = Vec::new();

    let repository_db = repository::parquet::Repository::load().unwrap_or_else(|e| {
        eprintln!("Failed to load repositories: {}", e.to_string());
        std::process::exit(1);
    });

    for pkg in package_names {
        let package = repository_db
            .find_package(pkg)
            .ok_or(CpsiError::PackageNotFound(pkg.clone()))?;

        install_packages.push(package);
    }

    let deps = dependency::resolve::resolve(&install_packages, &repository_db);

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
