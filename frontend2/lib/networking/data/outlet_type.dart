enum OutletType {
  any("Any socket", "Type1"),
  type2Vac230("Type2 - 230Vac", "Type2_230"),
  type2Vac400("Type2 - 400Vac", "Type2_400"),
  ccs("CCS", "CCS"),
  chademo("CHAdeMO", "CHAdeMO"),
  // type2Mennekes("Type2Mennekes"),
  // bar700SmallVehicles("700 bar small vehicles"),
  // schuko("Schuko"),
  ;

  const OutletType(this.name, this.rustId);
  final String name;
  final String rustId;
}
