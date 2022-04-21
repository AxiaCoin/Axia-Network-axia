# Axia

Implementation of a <https://axia.network> node in Rust based on the Axlib framework.

> **NOTE:** In 2018, we split our implementation of "Axia" from its development framework
> "Axlib". See the [Axlib][axlib-repo] repo for git history prior to 2018.

[axlib-repo]: https://github.com/axiatech/axlib

This repo contains runtimes for the Axia, AxiaTest, and Alphanet networks. The README provides
information about installing the `axia` binary and developing on the codebase. For more
specific guides, like how to be a validator, see the
[Axia Wiki](https://wiki.axia.network/docs/getting-started).

## Installation

If you just wish to run a Axia node without compiling it yourself, you may
either run the latest binary from our
[releases](https://github.com/axiatech/axia/releases) page, or install
Axia from one of our package repositories.

Installation from the Debian or rpm repositories will create a `systemd`
service that can be used to run a Axia node. This is disabled by default,
and can be started by running `systemctl start axia` on demand (use
`systemctl enable axia` to make it auto-start after reboot). By default, it
will run as the `axia` user.  Command-line flags passed to the binary can
be customized by editing `/etc/default/axia`. This file will not be
overwritten on updating axia. You may also just run the node directly from
the command-line.

### Debian-based (Debian, Ubuntu)

Currently supports Debian 10 (Buster) and Ubuntu 20.04 (Focal), and
derivatives. Run the following commands as the `root` user.

```bash
# Import the security@axia.io GPG key
gpg --recv-keys --keyserver hkps://keys.mailvelope.com 9D4B2B6EB8F97156D19669A9FF0812D491B96798
gpg --export 9D4B2B6EB8F97156D19669A9FF0812D491B96798 > /usr/share/keyrings/axia.gpg
# Add the Axia repository and update the package index
echo 'deb [signed-by=/usr/share/keyrings/axia.gpg] https://releases.axia.io/deb release main' > /etc/apt/sources.list.d/axia.list
apt update
# Install the `axia-keyring` package - This will ensure the GPG key
# used by APT remains up-to-date
apt install axia-keyring
# Install axia
apt install axia

```

### RPM-based (Fedora, CentOS)

Currently supports Fedora 32 and CentOS 8, and derivatives.

```bash
# Install dnf-plugins-core (This might already be installed)
dnf install dnf-plugins-core
# Add the repository and enable it
dnf config-manager --add-repo https://releases.axia.io/rpm/axia.repo
dnf config-manager --set-enabled axia
# Install axia (You may have to confirm the import of the GPG key, which
# should have the following fingerprint: 9D4B2B6EB8F97156D19669A9FF0812D491B96798)
dnf install axia
```

## Building

### Install via Cargo

Make sure you have the support software installed from the **Build from Source** section
below this section.

If you want to install Axia in your PATH, you can do so with with:

```bash
cargo install --git https://github.com/axiatech/axia --tag <version> axia --locked
```

### Build from Source

If you'd like to build from source, first install Rust. You may need to add Cargo's bin directory
to your PATH environment variable. Restarting your computer will do this for you automatically.

```bash
curl https://sh.rustup.rs -sSf | sh
```

If you already have Rust installed, make sure you're using the latest version by running:

```bash
rustup update
```

Once done, finish installing the support software:

```bash
sudo apt install build-essential git clang libclang-dev pkg-config libssl-dev
```

Build the client by cloning this repository and running the following commands from the root
directory of the repo:

```bash
git checkout <latest tagged release>
./scripts/init.sh
cargo build --release
```

Note that compilation is a memory intensive process. We recommend having 4 GiB of physical RAM or swap available (keep in mind that if a build hits swap it tends to be very slow).

#### Build from Source with Docker

You can also build from source using 
[Axia CI docker image](https://github.com/axiatech/scripts/tree/master/dockerfiles/ci-linux):

```bash
git checkout <latest tagged release>
docker run --rm -it -w /shellhere/axia \
                    -v $(pwd):/shellhere/axia \
                    axiatech/ci-linux:production cargo build --release
sudo chown -R $(id -u):$(id -g) target/
```

If you want to reproduce other steps of CI process you can use the following 
[guide](https://github.com/axiatech/scripts#gitlab-ci-for-building-docker-images).

## Networks

This repo supports runtimes for Axia, AxiaTest, and Alphanet.

### Connect to Axia Mainnet

Connect to the global Axia Mainnet network by running:

```bash
./target/release/axia --chain=axia
```

You can see your node on [telemetry] (set a custom name with `--name "my custom name"`).

[telemetry]: https://telemetry.axia.io/#list/Axia

### Connect to the "AxiaTest" Canary Network

Connect to the global AxiaTest canary network by running:

```bash
./target/release/axia --chain=axctest
```

You can see your node on [telemetry] (set a custom name with `--name "my custom name"`).

[telemetry]: https://telemetry.axia.io/#list/AxiaTest

### Connect to the Alphanet Testnet

Connect to the global Alphanet testnet by running:

```bash
./target/release/axia --chain=alphanet
```

You can see your node on [telemetry] (set a custom name with `--name "my custom name"`).

[telemetry]: https://telemetry.axia.io/#list/Alphanet

### Obtaining AXCs

If you want to do anything on Axia, AxiaTest, or Alphanet, then you'll need to get an account and
some AXC, AXCT, or WND tokens, respectively. See the
[claims instructions](https://claims.axia.network/) for Axia if you have AXCs to claim. For
Alphanet's WND tokens, see the faucet
[instructions](https://wiki.axia.network/docs/learn-AXC#getting-westies) on the Wiki.

## Hacking on Axia

If you'd actually like to hack on Axia, you can grab the source code and build it. Ensure you have
Rust and the support software installed. This script will install or update Rust and install the
required dependencies (this may take up to 30 minutes on Mac machines):

```bash
curl https://getaxlib.io -sSf | bash -s -- --fast
```

Then, grab the Axia source code:

```bash
git clone https://github.com/axiatech/axia.git
cd axia
```

Then build the code. You will need to build in release mode (`--release`) to start a network. Only
use debug mode for development (faster compile times for development and testing).

```bash
./scripts/init.sh   # Install WebAssembly. Update Rust
cargo build # Builds all native code
```

You can run the tests if you like:

```bash
cargo test --all --release
```

You can start a development chain with:

```bash
cargo run -- --dev
```

Detailed logs may be shown by running the node with the following environment variables set:

```bash
RUST_LOG=debug RUST_BACKTRACE=1 cargo run -- --dev
```

### Development

You can run a simple single-node development "network" on your machine by running:

```bash
axia --dev
```

You can muck around by heading to <https://axia.js.org/apps> and choose "Local Node" from the
Settings menu.

### Local Two-node Testnet

If you want to see the multi-node consensus algorithm in action locally, then you can create a
local testnet. You'll need two terminals open. In one, run:

```bash
axia --chain=axia-local --alice -d /tmp/alice
```

And in the other, run:

```bash
axia --chain=axia-local --bob -d /tmp/bob --port 30334 --bootnodes '/ip4/127.0.0.1/tcp/30333/p2p/ALICE_BOOTNODE_ID_HERE'
```

Ensure you replace `ALICE_BOOTNODE_ID_HERE` with the node ID from the output of the first terminal.

### Monitoring

[Setup Prometheus and Grafana](https://wiki.axia.network/docs/maintain-guides-how-to-monitor-your-node).

Once you set this up you can take a look at the [Axia Grafana dashboards](grafana/README.md) that we currently maintain. 

### Using Docker

[Using Docker](doc/docker.md)

### Shell Completion

[Shell Completion](doc/shell-completion.md)

## Contributing

### Contributing Guidelines

[Contribution Guidelines](CONTRIBUTING.md)

### Contributor Code of Conduct

[Code of Conduct](CODE_OF_CONDUCT.md)

## License

Axia is [GPL 3.0 licensed](LICENSE).
