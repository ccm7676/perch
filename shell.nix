{ pkgs ? import <nixpkgs> {} }:
  
  pkgs.mkShell {
    buildInputs = with pkgs; [
	];
  
  nativeBuildInputs = with pkgs.buildPackages; [
    sqlitecpp
  ];
    
}
