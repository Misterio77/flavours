{ mkShell, callPackage, rustc, rust-analyzer, rustfmt, clippy, ... }:
mkShell {
  inputsFrom = [ (callPackage ./. { }) ];
  buildInputs = [
    rustc
    rust-analyzer
    rustfmt
    clippy
  ];
}
