#[derive(Debug)]
pub enum PgnCategory {
    Mandatory,
    General,
    Proprietary,
    Power,
    Steering,
    Propulsion,
    Navigation,
    Ais,
    Environmental,
    Entertainment,
    Other,
}

#[derive(Debug)]
pub enum FieldType {
    Variable,
    NotUsed,
    Lookup,
    Integer,
    Decimal,
    Float,
    AsciiString,
    FixedString,
    PascalString,
    WideString,
}

#[derive(Debug)]
pub enum Unit {
    Volts,
    Hertz,
    Seconds,
    Degrees,
    DegreesCelcius,
    Radians,
    RadiansPerSecond,
    Watts,
    WattHours,
    KilowattHours,
    VoltAmps,
    VoltAmpsReactive,

}

/// The `Pgn` type holds information pertaining to a specific PGN and its fields. Technically, this
/// is a Parameter Group and not a Parameter Group Number, but colloquially Parameter Groups are
/// referred to as PGNs.
#[derive(Debug)]
pub struct Pgn {
    /// Name of the pgn. Primarity of use for documentation and debugging.
    pub name: &'static str,
    /// The category the pgn belongs to. See [PgnCategory](enum.PgnCategory.html) Enum for possible
    /// values.
    pub category: PgnCategory,
    /// Integer ID of the pgn.
    pub pgn: u32,
    /// Flag indicating that this PGN has been confidently reverse engineered. If false, then the
    /// structure of the PGN is a best guess.
    pub is_known: bool,
    /// Length of the PGN in bytes. An 8-byte PGN is single frame, 9-252 is Fast Packet and larger
    /// than that is rare and require the use of the J1939 transport protocol.
    pub size: u32,
    /// How many fields repeat, counting back from the end of the definition.
    pub repeating_fields: u32,
    /// Vector of [Field](struct.Field.html) Structs.
    pub fields: Vec<Field>
}

/// The `Field` type holds information pertaining to a specific field in a PGN
#[derive(Default, Debug)]
pub struct Field {
    /// Name of the field. Primarily of use for documentation and debugging.
    pub name: &'static str,
    /// Description of the field. Rarely used, again primarily for documentation and info for
    /// humans.
    pub description: Option<&'static str>,
    /// Unit of measure for the value in the field. See [Unit](enum.Unit.html) Enum for possibe values.
    /// If the field is unitless, use `None`.
    pub unit: Option<Unit>,
    /// Indicates how the field should be parsed. I.e. whether it is a floating point value, an
    /// integer, etc. See [FieldType](enum.FieldType.html) Enum for possible values.
    pub field_type: Option<FieldType>,
    /// Bit offset from the beginning of the reassembled NMEA 2000 packet. Many PGNs will be a
    /// single frame and will fit in a single 64-bit integer. However, some are much larger.
    /// Reassembling a field which crosses a 64-bit boundary is complex.
    pub start: u16,
    /// How many bits long the field is.
    pub size: u16,
    /// Most data is encoded as an integer value on the wire because the protocol is designed for
    /// small microcontrollers which may not have floating point hardware. For instance, a voltage
    /// measurement may be in 100ths of a volt, so a value of 1205 would be 12.05V. In this case,
    /// the multiplier value would be 0.01.
    pub multiplier: f64,
    /// Excess-K offset. See [Offset Binary](http://wikipedia.org/wiki/offset_binary).
    pub offset: i64
}

/// Constructs a list of `Pgn`s.
///
/// Right now, returns a `Vec<Pgn>` structure, but a BTree or something with faster lookup times
/// may be appropriate. The returned data structure is accessed every time a new PGN arrives on the
/// wire.
///
/// # Examples
///
/// ```
/// use libnmea::*
///
/// let pgns = pgn_list();
///
/// println!("{:?}", pgns);
/// ```
pub fn pgn_list() -> Vec<Pgn> {
    let pgn_list = vec![
        Pgn {
            name: "Unknown PGN",
            category: PgnCategory::Mandatory,
            pgn: 0,
            is_known: false,
            size: 8,
            repeating_fields: 0,
            fields: vec![
                Field {
                    name: "Manufacturer Code",
                    field_type: Some(FieldType::Lookup),
                    start:0,
                    size: 11,
                    ..Default::default()
                },
                // Two bits reserved
                Field {
                    name: "Industry Code",
                    field_type: Some(FieldType::Lookup),
                    start: 13,
                    size: 3,
                    ..Default::default()
                },
            ]
        },
        Pgn {
            name: "ISO Acknowledgement",
            category: PgnCategory::Mandatory,
            pgn: 59392,
            is_known: true,
            size: 8,
            repeating_fields: 0,
            fields: vec![
                Field {
                    name: "Control",
                    field_type: Some(FieldType::Lookup),
                    start: 0,
                    size: 8,
                    ..Default::default()
                },
                Field {
                    name: "Group Function",
                    start: 8,
                    size: 8,
                    ..Default::default()
                },
                // 24 bits reserved
                Field {
                    name: "PGN",
                    description: Some("Parameter group number of requested information"),
                    start: 40,
                    size: 24,
                    field_type: Some(FieldType::Integer),
                    ..Default::default()
                },
            ]
        },
        Pgn {
            name: "ISO Request",
            category: PgnCategory::Mandatory,
            pgn: 59904,
            is_known: true,
            size: 3,
            repeating_fields: 0,
            fields: vec![
                Field {
                    name: "PGN",
                    description: Some("Parameter group number of requested information"),
                    start: 40,
                    size: 24,
                    field_type: Some(FieldType::Integer),
                    ..Default::default()
                },
            ],
        },
    ];

    pgn_list
}
