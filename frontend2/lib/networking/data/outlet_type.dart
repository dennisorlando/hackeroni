enum OutletType {
  any("Any socket"),
  ccs("CCS"),
  type2Vac230("Type2 - 230Vac"),
  type2Vac400("Type2 - 400Vac"),
  type2Mennekes("Type2Mennekes"),
  bar700SmallVehicles("700 bar small vehicles"),
  schuko("Schuko"),
  chademo("CHAdeMO"),
  ;

  const OutletType(this.name);
  final String name;
}
