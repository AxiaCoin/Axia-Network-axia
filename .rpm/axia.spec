%define debug_package %{nil}

Name: axia
Summary: Implementation of a https://axia.network node in Rust based on the Axlib framework.
Version: @@VERSION@@
Release: @@RELEASE@@%{?dist}
License: GPLv3
Group: Applications/System
Source0: %{name}-%{version}.tar.gz

Requires: systemd, shadow-utils
Requires(post): systemd
Requires(preun): systemd
Requires(postun): systemd

BuildRoot: %{_tmppath}/%{name}-%{version}-%{release}-root

%description
%{summary}


%prep
%setup -q


%install
rm -rf %{buildroot}
mkdir -p %{buildroot}
cp -a * %{buildroot}

%post
config_file="/etc/default/axia"
getent group axia >/dev/null || groupadd -r axia
getent passwd axia >/dev/null || \
    useradd -r -g axia -d /home/axia -m -s /sbin/nologin \
    -c "User account for running axia as a service" axia
if [ ! -e "$config_file" ]; then
    echo 'AXIA_CLI_ARGS=""' > /etc/default/axia
fi
exit 0

%clean
rm -rf %{buildroot}

%files
%defattr(-,root,root,-)
%{_bindir}/*
/usr/lib/systemd/system/axia.service
