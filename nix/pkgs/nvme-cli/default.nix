{ stdenv, lib, libuuid, pkg-config, sources }:

stdenv.mkDerivation rec {
  version = sources.nvme-cli.rev;
  name = "nvme-cli-${version}";
  src = sources.nvme-cli;

  nativeBuildInputs = [ pkg-config ];
  buildInputs = [ libuuid ];

  makeFlags = [ "DESTDIR=$(out)" "PREFIX=" ];
  # Required to build with latest nixpkgs with adds more warning/error
  NIX_CFLAGS_COMPILE = [ "-Wno-enum-int-mismatch" "-Wno-array-bounds" "-Wno-use-after-free" ];

  # To omit the hostnqn and hostid files that are impure and should be unique
  # for each target host:
  installTargets = [ "install-spec" ];

  meta = with lib; {
    description = sources.nvme-cli.description;
    homepage = sources.nvme-cli.homepage;
    license = licenses.gpl2Plus;
    platforms = platforms.linux;
  };
}
