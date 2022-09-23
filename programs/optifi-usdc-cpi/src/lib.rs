anchor_gen::generate_cpi_crate!("idl.json");

#[cfg(not(feature = "devnet"))]
declare_id!("opucrGSP5hyYAEfQMjYrchfLtwowjiAjAMwUmmjoYGP");

#[cfg(feature = "devnet")]
declare_id!("oudvxi2FrrgAS8boTYg4XWVkNsDE8ASK4AveVZGEjQ4");
