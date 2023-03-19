![alt text](https://media.discordapp.net/attachments/1055231169479843870/1086803851056783481/cywf_ultra_realistic_Network_Ninja_in_a_cyberpunk_neon_age_who__d9457b3c-feb9-4969-b7d1-4e5d5e1dac3e.png?width=1138&height=1138)

# NetNinja

Streamline Linux server troubleshooting with NetNinja - the ultimate tool for network and system admins. Automate common checks like ping and process monitoring. Join the open-source community and contribute to the power of NetNinja!

# Installation

To install NetNinja, clone the repository and run the `install.sh` script:

```sh
git clone https://github.com/your-username/netninja.git
cd netninja
./install.sh
```

This will create a symbolic link to the netninja executable in `/usr/local/bin`.

## Usage

To use NetNinja, simply run the netninja command with any options or arguments:

```sh
netninja --help
```

This will display a list of available options and commands.
Making NetNinja Available System-wide

To make NetNinja available system-wide, you can add a symbolic link to the netninja executable in a directory that is in the system's PATH environment variable.

For example, you can create a symbolic link to netninja in `/usr/local/bin`, which is a common directory for user-installed executables on Linux systems:

```sh
ln -s /path/to/netninja/bin/netninja /usr/local/bin/netninja
```

This will create a symbolic link to the netninja executable in `/usr/local/bin`, which allows you to run the netninja command from any directory on your system.

Note that you will need to replace `/path/to/netninja` with the actual path to your NetNinja directory. You will also need to have superuser privileges (sudo) to create the symbolic link in the /usr/local/bin directory.
Contributing

If you would like to contribute to NetNinja, please see the [CONTRIBUTING.md](https://github.com/cywf/netninja/docs/CONTRIBUTING.md) file for guidelines on how to contribute. We welcome bug reports, feature requests, and pull requests!
License

NetNinja is licensed under the [MIT License](https://github.com/cywf/netninja/docs/LICENSE.txt).
