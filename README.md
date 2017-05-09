APT Packages Diff
=================

`apt-packages-diff` is a tool written in Rust that find changes between
different Packages files from any Debian-based distro in local plain,
gzipped or remote locations.

This is also useful for comparing changes in Cydia repositories and
be able to determine what's new, removed or changed.

This is NOT an official NowSecure product.

Author
------

Written by Sergi Àlvarez i Capilla at NowSecure <pancake@nowsecure.com>

Build and install
-----------------

This thing is compiled with Cargo:

	cargo build --release

To install just copy the binary in `target/release/apt-packages-diff` to your $PATH or:

	cargo install --release

Crosscompilation
----------------

It is also possible to build apt-packages-diff for iOS using the following line:

	cargo build --release --target aarch64-apple-ios
	scp target/aarch64-apple-ios/apt-packages-diff root@192.168.1.40:/usr/bin

ssh root@192.168.1.40

	# cd /usr/bin
	# chmod +x apt-packages-diff
	# ldid -S apt-packages-diff

How to use
----------

You must provide the old and new Packages file to display the changes:

	apt-packages-diff /var/lib/apt/lists/apt.saurik.com_dists_ios_1240.10_Release \
		"http://apt.saurik.com/dists/ios/main/binary-iphoneos-arm/Packages"

Apt caches those files into `/var/lib/apt/lists/*._Packages`.
