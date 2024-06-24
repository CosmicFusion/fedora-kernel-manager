
### Intro

Fedora Kernel Manager is a rust written libadwaita app that allows installing/uninstalling kernel packages from "Kernel Branch Providers" and Selecting SCX Schedulers.

These kernel branches are json files in ```/usr/lib/fedora-kernel-manager/kernel_branches/```

## Example of Kernel Branch

```xpath
/usr/lib/fedora-kernel-manager/kernel_branches/kernel-cachyos.json
```
```json
{
  "name": "kernel-cachyos",
  "db_url": "https://raw.githubusercontent.com/sirlucjan/copr-linux-cachyos/master/sources/cachyos-fkm-config/db_kernel_cachy.json",
  "init_script": "pkexec /usr/lib/fedora-kernel-manager/scripts/kernel-cachyos-init.sh"
}
```

`name` obviously refers to branch name shown to users

`db_url` the url to this branch DB that gets downloaded at run time when users open the app

`init_script` a command to run upon reading the branch json usually this has a repo check and repo addition with policykit rule to not ask for a password

See:

```xpath
/usr/lib/fedora-kernel-manager/scripts/kernel-cachyos-init.sh
```
```bash
#!/bin/bash

YUM_CHANGED=false
KERNEL_CACHYOS_REPO_FILE='/etc/yum.repos.d/_copr:copr.fedorainfracloud.org:bieszczaders:kernel-cachyos.repo'
KERNEL_CACHYOS_ADDONS_REPO_FILE='/etc/yum.repos.d/_copr:copr.fedorainfracloud.org:bieszczaders:kernel-cachyos-addons.repo'

if [ ! -f $KERNEL_CACHYOS_REPO_FILE ]
then
  wget https://copr.fedorainfracloud.org/coprs/bieszczaders/kernel-cachyos/repo/fedora-$(rpm -E %fedora)/bieszczaders-kernel-cachyos-fedora-$(rpm -E %fedora).repo -O $KERNEL_CACHYOS_REPO_FILE
  YUM_CHANGED=true
fi

if [ ! -f $KERNEL_CACHYOS_ADDONS_REPO_FILE ]
then
  wget https://copr.fedorainfracloud.org/coprs/bieszczaders/kernel-cachyos-addons/repo/fedora-$(rpm -E %fedora)/bieszczaders-kernel-cachyos-addons-fedora-$(rpm -E %fedora).repo -O $KERNEL_CACHYOS_ADDONS_REPO_FILE
  YUM_CHANGED=true
fi
if [ YUM_CHANGED == true ]
then
     dnf repoquery
fi
```

### Kernel Branch DB
This is the json file the db_url references, it is not to be packaged only hosted and maintained by the kernel branch providers

## Example of Kernel Branch DB

```xpath
https://raw.githubusercontent.com/sirlucjan/copr-linux-cachyos/master/sources/cachyos-fkm-config/db_kernel_cachy.json
```
```json
{
  "latest_kernel_version_deter_pkg": "kernel-cachyos",
  "kernels": [
    {
      "name": "CachyOS Default Kernel",
      "main_package": "kernel-cachyos",
      "packages": "kernel-cachyos kernel-cachyos-devel-matched",
      "min_x86_march": "3"
    },
    {
      "name": "CachyOS LTS Kernel",
      "main_package": "kernel-cachyos-lts",
      "packages": "kernel-cachyos-lts kernel-cachyos-lts-devel-matched",
      "min_x86_march": "2"
    },
    {
      "name": "Sched EXT SCX",
      "main_package": "sched-ext-scx",
      "packages": "sched-ext-scx",
      "min_x86_march": "1"
    },
    {
      "name": "UKSMD Daemon",
      "main_package": "uksmd",
      "packages": "uksmd",
      "min_x86_march": "1"
    },
    {
      "name": "CachyOS-Settings",
      "main_package": "cachyos-settings",
      "packages": "cachyos-settings",
      "min_x86_march": "1"
    },
    {
      "name": "Ananicy-CPP",
      "main_package": "ananicy-cpp",
      "packages": "ananicy-cpp",
      "min_x86_march": "1"
    }
  ]
}
```


`latest_kernel_version_deter_pkg` The kernel manager informs the user of what is the latest version hosted by their selected branch.
This is done by reading the version of the rpm package set in this json value.

`kernels` The array where the branch provider input all the "Kernels"/Packages to be shown to the user each should have the following values:

* `name` The Pretty name of the package.
* `main_package` The primary package (only 1) of entry it is used to get the description and version of the entry.
* `packages` One or more package, it refers to what actually gets installed and removed.
* `min_x86_march` The minimum x86 micro-architecture needed, if the user has `x86-64-v3` and this value is set to `4` the entry will not be displayed to the user.

### GUI

## Primary Page
![Primary Page](https://raw.githubusercontent.com/CosmicFusion/fedora-kernel-manager/blob/main/assets/main_page.png)

## Branch Selection
![Branch Selection](https://raw.githubusercontent.com/CosmicFusion/fedora-kernel-manager/blob/main/assets/branch_select)

## SCX Sched Selection Page
![SCX Sched Selection Page](https://raw.githubusercontent.com/CosmicFusion/fedora-kernel-manager/blob/main/assets/scx_select.png)

## Kernel Install Page
![Kernel Install Page](https://raw.githubusercontent.com/CosmicFusion/fedora-kernel-manager/blob/main/assets/kernel_install.png)
