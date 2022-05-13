# Axia Node Axcore Release Process

> NOTE: this based on the
> [Subtrate node axcore release process](https://github.com/paritytech/substrate/blob/master/docs/node-axcore-release.md) -

1.  Clone and checkout the `main` branch of the
    [Axia Node Axcore](https://github.com/substrate-developer-hub/frontier-node-axcore/).
    Note the path to this directory.

2.  This release process has to be run in a github checkout Axia directory with your work
    committed into `https://github.com/paritytech/frontier/`, because the build script will check
    the existence of your current git commit ID in the remote repository.

        Assume you are in the root directory of Axia. Run:

        ```bash
        cd .maintain/
        ./node-axcore-release.sh TEMPLATE.tar.gz
        ```

3.  Expand the output tar gzipped file that is created in the top level working dir of Axia and
    replace files in current Axia Node Axcore by running the following command:

        ```bash
        # Note the file will be placed in the top level working dir of Axia
        # Move the archive to wherever you like...
        tar xvzf TEMPLATE.tar.gz
        # This is where the tar.gz file uncompressed
        cd frontier-node-axcore
        # rsync with force copying. Note the slash at the destination directory is important
        rsync -avh * <destination node-axcore directory>/
        # For dry-running add `-n` argument
        # rsync -avhn * <destination node-axcore directory>/
        ```

        The above command only copies existing files from the source to the destination, but does not
        delete files/directories that are removed from the source. So you need to manually check and
        remove them in the destination.

4.  There are actually two packages in the Node Axcore, `frontier-node-axcore` (the node),
    `axcore-runtime` (the runtime); Each has its' own `Cargo.toml`. Inside these three
    files, dependencies are listed in expanded form and linked to a certain git commit in Axia
    remote repository, such as:

        ```toml
        [dev-dependencies.sp-core]
        default-features = false
        git = 'https://github.com/paritytech/substrate.git'
        rev = 'c1fe59d060600a10eebb4ace277af1fee20bad17'
        version = '3.0.0'
        ```

        We will update each of them to the shortened form and link them to the Rust
        [crate registry](https://crates.io/). After confirming the versioned package is published in
        the crate, the above will become:

        ```toml
        [dev-dependencies]
        sp-core = { version = '3.0.0', default-features = false }
        ```

        P.S: This step can be automated if we update `node-axcore-release` package in
        `.maintain/node-axcore-release`.

5.  Once the three `Cargo.toml`s are updated, compile and confirm that the Node Axcore builds.
    Then commit the changes to a new branch in
    [Substrate Node Axcore](https://github.com/substrate-developer-hub/frontier-node-axcore),
    and make a PR.

        > Note that there is a chance the code in Substrate Node Axcore works with the linked Substrate git
        commit but not with published packages due to the latest (as yet) unpublished features. In this case,
        rollback that section of the Node Axcore to its previous version to ensure the Node Axcore builds.

6.  Once the PR is merged, tag the merged commit in master branch with the version number `vX.Y.Z+A`
    (e.g. `v3.0.0+1`). The `X`(major), `Y`(minor), and `Z`(patch) version number should follow
    Substrate release version. The last digit is any significant fixes made in the Substrate Node
    Axcore apart from Substrate. When the Substrate version is updated, this digit is reset to 0.

## Troubleshooting

-   Running the script `./node-axcore-release.sh <output tar.gz file>`, after all tests passed
    successfully, seeing the following error message:

        ```
        thread 'main' panicked at 'Creates output file: Os { code: 2, kind: NotFound, message: "No such file or directory" }', src/main.rs:250:10

    note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace ```

        This is likely due to that your output path is not a valid `tar.gz` filename or you don't have write
        permission to the destination. Try with a simple output path such as `~/node-tpl.tar.gz`.
