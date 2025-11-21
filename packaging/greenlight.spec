Name:           greenlight
Version:        0.1.0
Release:        %{?release_tag}%{!?release_tag:1}%{?dist}
Summary:        Boot-time validation tool for embedded/edge systems
License:        Apache-2.0
URL:            https://github.com/SamD2021/greenlight
Group:          System Environment/Daemons
Packager:       Greenlight CI
BuildArch:      x86_64

%description
Greenlight is a small boot-time validation tool intended to run early in the
boot process and perform system checks. This package installs the binary,
systemd units, and a default configuration into /etc/greenlight.

%prep
# no sources to unpack

%build
# no build step; packaging a pre-built static binary

%install
rm -rf %{buildroot}
mkdir -p %{buildroot}/usr/bin
# binary_path is passed from rpmbuild invocation
if [ -z "%{?binary_path}" ]; then
  echo "ERROR: binary_path not defined" >&2
  exit 1
fi
install -m 0755 %{?binary_path} %{buildroot}/usr/bin/greenlight

mkdir -p %{buildroot}/usr/lib/systemd/system
install -m 0644 %{?sourcedir}/packaging/greenlight-required.service %{buildroot}/usr/lib/systemd/system/greenlight-required.service
install -m 0644 %{?sourcedir}/packaging/greenlight-wanted.service %{buildroot}/usr/lib/systemd/system/greenlight-wanted.service

mkdir -p %{buildroot}/etc/greenlight
install -m 0644 %{?sourcedir}/packaging/config.toml %{buildroot}/etc/greenlight/config.toml

%post
%systemd_post greenlight-required.service
%systemd_post greenlight-wanted.service

%postun
%systemd_postun_with_restart greenlight-required.service
%systemd_postun_with_restart greenlight-wanted.service

%files
%defattr(-,root,root,-)
/usr/bin/greenlight
/usr/lib/systemd/system/greenlight-required.service
/usr/lib/systemd/system/greenlight-wanted.service
%config(noreplace) /etc/greenlight/config.toml

%changelog
* Thu Nov 20 2025 Samuel Dasilva <samuelramos852@gmail.com> - 0.1.0-1
- Initial RPM
