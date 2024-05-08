{ pkgs ? import <nixpkgs> {} }:
  
  pkgs.mkShell {
    buildInputs = with pkgs; [
      dbus
	];
  
  nativeBuildInputs = with pkgs.buildPackages; [
    pkg-config
	  dbus
	  glib
    pango
    gdk-pixbuf
    graphene
    gtk4
	  gtk4-layer-shell
    sqlitecpp
  ];
    
}
