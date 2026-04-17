{
  description = "Rust development environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };

        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rust-analyzer" "clippy" "rustfmt" ];
          targets = [ "wasm32-unknown-unknown" ];
        };

        opencascade = pkgs.opencascade-occt;

        # Mesa drivers expose Vulkan and OpenGL ICDs. vulkan-loader is the
        # runtime that wgpu uses to discover them via the ICD JSON files.
        vulkanPkgs = [
          pkgs.vulkan-loader
          pkgs.vulkan-validation-layers
          pkgs.mesa
          pkgs.mesa.drivers
        ];
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = [
            # Rust
            rustToolchain
            pkgs.cargo-watch

            # C/C++ toolchain (fixes "linker `cc` not found")
            pkgs.gcc
            pkgs.gnumake
            pkgs.cmake
            pkgs.pkg-config

            # OpenCASCADE (for occt-sys)
            opencascade

            # Common occt-sys dependencies
            pkgs.freetype
            pkgs.fontconfig
            pkgs.libGL
            pkgs.libGLU
            pkgs.libX11
            pkgs.libXcursor
            pkgs.libXrandr
            pkgs.libXmu
            pkgs.libXi
            pkgs.libXext
            pkgs.libxkbcommon

            # Vulkan / wgpu
          ] ++ vulkanPkgs ++ [

            pkgs.libpng
            pkgs.libjpeg
            pkgs.libtiff
            pkgs.tbb
            pkgs.vtk
          ];

          shellHook = ''
            export OPENCASCADE_ROOT="${opencascade}"
            export PKG_CONFIG_PATH="${opencascade}/lib/pkgconfig:$PKG_CONFIG_PATH"
            export LD_LIBRARY_PATH="${opencascade}/lib:${pkgs.freetype}/lib:${pkgs.fontconfig}/lib:${pkgs.libX11}/lib:${pkgs.libXcursor}/lib:${pkgs.libXrandr}/lib:${pkgs.libXi}/lib:${pkgs.libxkbcommon}/lib:${pkgs.vulkan-loader}/lib:${pkgs.mesa}/lib:$LD_LIBRARY_PATH"
            export LIBRARY_PATH="${opencascade}/lib:$LIBRARY_PATH"

            # Point the Vulkan loader at Mesa's ICD JSON so it can find the
            # actual GPU driver. Without this wgpu sees no backends at all.
            export VK_ICD_FILENAMES="${pkgs.mesa.drivers}/share/vulkan/icd.d/intel_icd.x86_64.json:${pkgs.mesa.drivers}/share/vulkan/icd.d/radeon_icd.x86_64.json"

            # Uncomment to force wgpu to use the GL backend via ANGLE/Mesa
            # instead of Vulkan — useful if Vulkan still fails:
            # export WGPU_BACKEND=gl

            # CMake 4.x removed compatibility with CMakeLists.txt files that
            # declare a pre-3.5 minimum version. occt-sys vendors one such file.
            # CMAKE_POLICY_VERSION_MINIMUM is read directly by CMake 4.x itself,
            # so setting it in the environment is sufficient — no -D flag needed.
            export CMAKE_POLICY_VERSION_MINIMUM=3.5
          '';
        };
      });
}
