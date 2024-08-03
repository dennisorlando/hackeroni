enum OutletType {
  any("Any socket"),
  type2Vac230("Type2 - 230Vac"),
  type2Vac400("Type2 - 400Vac"),
  ccs("CCS"),
  chademo("CHAdeMO"),
  // type2Mennekes("Type2Mennekes"),
  // bar700SmallVehicles("700 bar small vehicles"),
  // schuko("Schuko"),
  ;

  const OutletType(this.name);
  final String name;
}
