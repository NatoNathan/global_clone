%define __spec_install_post %{nil}
%define __os_install_post %{_dbpath}/brp-compress
%define debug_package %{nil}

Name: global_clone
Summary: Global_clone is a tool for cloning a repository to templated paths
Version: @@VERSION@@
Release: @@RELEASE@@%{?dist}
License: MIT License
Group: Applications/System
Source0: %{name}-%{version}.tar.gz
URL: https://github.com/natonathan/global_clone

BuildRoot: %{_tmppath}/%{name}-%{version}-%{release}-root

%description
%{summary}

%prep
%setup -q

%install
rm -rf %{buildroot}
mkdir -p %{buildroot}
cp -a * %{buildroot}

%clean
rm -rf %{buildroot}

%files
%defattr(-,root,root,-)
%{_bindir}/*
