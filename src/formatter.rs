extern crate nom;

use nom::number::complete::{be_u128, be_u16, be_u32, be_u64};
use parser;
use rustc_hash::FxHashMap as HashMap;

/// conversion of array of bytes to various DataRecordValues
#[inline]
pub fn be_int(s: &[u8]) -> parser::DataRecordValue {
    match s.len() {
        1 => parser::DataRecordValue::U8(s[0]),
        2 => match read_u16(s).ok() {
            Some((_, val)) => parser::DataRecordValue::U16(val),
            None => parser::DataRecordValue::Bytes(s),
        },
        4 => match read_u32(s).ok() {
            Some((_, val)) => parser::DataRecordValue::U32(val),
            None => parser::DataRecordValue::Bytes(s),
        },
        8 => match read_u64(s).ok() {
            Some((_, val)) => parser::DataRecordValue::U64(val),
            None => parser::DataRecordValue::Bytes(s),
        },
        _ => parser::DataRecordValue::Bytes(s),
    }
}

/// conversion of bytes array to a DataRecordValue ipv4
#[inline]
pub fn ipv4_addr(s: &[u8]) -> parser::DataRecordValue {
    match read_u32(s).ok() {
        Some((_, ipv4)) => parser::DataRecordValue::IPv4(ipv4.into()),
        None => parser::DataRecordValue::Bytes(s),
    }
}

/// conversion of bytes array to a DataRecordValue ipv6
#[inline]
pub fn ipv6_addr(s: &[u8]) -> parser::DataRecordValue {
    match read_u128(s).ok() {
        Some((_, ipv6)) => parser::DataRecordValue::IPv6(ipv6.into()),
        None => parser::DataRecordValue::Bytes(s),
    }
}

/// conversion of bytes to a DataRecordValue string
#[inline]
pub fn be_string(s: &[u8]) -> parser::DataRecordValue {
    parser::DataRecordValue::String(String::from_utf8_lossy(s).to_string())
}

#[inline]
named!(read_u16<u16>, call!(be_u16));
#[inline]
named!(read_u32<u32>, call!(be_u32));
#[inline]
named!(read_u64<u64>, call!(be_u64));
#[inline]
named!(read_u128<u128>, call!(be_u128));

#[inline]
fn mpls_stack(s: &[u8]) -> parser::DataRecordValue {
    //      0                   1                   2
    //  0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3
    // +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
    // |                Label                  | Exp |S|
    // +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+

    // Label:  Label Value, 20 bits
    // Exp:    Experimental Use, 3 bits
    // S:      Bottom of Stack, 1 bit

    named!(parse_mpls_stack <&[u8], (u32, u8, u8)>, bits!(
        tuple!(
            take_bits!( 1u32 ),
            take_bits!( 1u8 ),
            take_bits!( 1u8 )
        )
    ));

    match parse_mpls_stack(s) {
        Ok((_, (label, exp, bottom))) => parser::DataRecordValue::MPLS(label, exp, bottom),
        Err(err) => parser::DataRecordValue::Err(format!("{}", err), s),
    }
}

/// mapping of field_id -> parser
pub type FieldFormatter = HashMap<u16, (&'static str, fn(&[u8]) -> parser::DataRecordValue)>;

/// mapping of enterprise_number -> FieldFormatters
pub type EnterpriseFormatter = HashMap<u32, FieldFormatter>;

/// Fieldparser create a map of field parser
#[macro_export]
macro_rules! field_parser(
    { $($key:expr => ($string:expr, $value:expr)),+ } => {
        {
        let mut m = FieldFormatter::default();
            $(
                m.insert($key, ($string, $value));
            )+
            m
        }
    };
);

/// default field_parsers for enterprise number 0
pub fn get_default_parsers() -> FieldFormatter {
    field_parser! {
        1 => ("octetDeltaCount", be_int),
        2 => ("packetDeltaCount", be_int),
        4 => ("protocolIdentifier", be_int),
        5 => ("classOfServiceIPv4", be_int),
        6 => ("tcpControlBits", be_int),
        7 => ("sourceTransportPort", be_int),
        8 => ("sourceIPv4Address", ipv4_addr),
        9 => ("sourceIPv4Mask", be_int),
        10 => ("ingressInterface", be_int),
        11 => ("destinationTransportPort", be_int),
        12 => ("destinationIPv4Address", ipv4_addr),
        13 => ("destinationIPv4Mask", be_int),
        14 => ("egressInterface", be_int),
        15 => ("ipNextHopIPv4Address", ipv4_addr),
        16 => ("bgpSourceAsNumber", be_int),
        17 => ("bgpDestinationAsNumber", be_int),
        18 => ("bgpNextHopIPv4Address", be_int),
        19 => ("postMCastPacketDeltaCount", be_int),
        20 => ("postMCastOctetDeltaCount", be_int),
        21 => ("flowEndSysUpTime", be_int),
        22 => ("flowStartSysUpTime", be_int),
        23 => ("postOctetDeltaCount", be_int),
        24 => ("postPacketDeltaCount", be_int),
        25 => ("minimumPacketLength", be_int),
        26 => ("maximumPacketLength", be_int),
        27 => ("sourceIPv6Address", ipv6_addr),
        28 => ("destinationIPv6Address", ipv6_addr),
        29 => ("sourceIPv6Mask", be_int),
        30 => ("destinationIPv6Mask", be_int),
        31 => ("flowLabelIPv6", be_int),
        32 => ("icmpTypeCodeIPv4", be_int),
        33 => ("igmpType", be_int),
        36 => ("flowActiveTimeOut", be_int),
        37 => ("flowInactiveTimeout", be_int),
        40 => ("exportedOctetTotalCount", be_int),
        41 => ("exportedMessageTotalCount", be_int),
        42 => ("exportedFlowTotalCount", be_int),
        44 => ("sourceIPv4Prefix", be_int),
        45 => ("destinationIPv4Prefix", be_int),
        46 => ("mplsTopLabelType", be_int),
        47 => ("mplsTopLabelIPv4Address", ipv4_addr),
        52 => ("minimumTtl", be_int),
        53 => ("maximumTtl", be_int),
        54 => ("identificationIPv4", be_int),
        55 => ("postClassOfServiceIPv4", be_int),
        56 => ("sourceMacAddress", be_int),
        57 => ("postDestinationMacAddr", be_int),
        58 => ("vlanId", be_int),
        59 => ("postVlanId", be_int),
        60 => ("ipVersion", be_int),
        62 => ("ipNextHopIPv6Address", ipv6_addr),
        63 => ("bgpNextHopIPv6Address", ipv6_addr),
        64 => ("ipv6ExtensionHeaders", be_int),
        70 => ("mplsTopLabelStackEntry", mpls_stack),
        71 => ("mplsLabelStackEntry2", mpls_stack),
        72 => ("mplsLabelStackEntry3", mpls_stack),
        73 => ("mplsLabelStackEntry4", mpls_stack),
        74 => ("mplsLabelStackEntry5", mpls_stack),
        75 => ("mplsLabelStackEntry6", mpls_stack),
        76 => ("mplsLabelStackEntry7", mpls_stack),
        77 => ("mplsLabelStackEntry8", mpls_stack),
        78 => ("mplsLabelStackEntry9", mpls_stack),
        79 => ("mplsLabelStackEntry10", mpls_stack),
        80 => ("destinationMacAddress", be_int),
        81 => ("postSourceMacAddress", be_int),
        82 => ("interfaceName", be_int),
        83 => ("interfaceDescription", be_int),
        84 => ("samplerName", be_int),
        85 => ("octetTotalCount", be_int),
        86 => ("packetTotalCount", be_int),
        88 => ("fragmentOffsetIPv4", be_int),
        128 => ("bgpNextAdjacentAsNumber", be_int),
        129 => ("bgpPrevAdjacentAsNumber", be_int),
        130 => ("exporterIPv4Address", ipv4_addr),
        131 => ("exporterIPv6Address", ipv6_addr),
        132 => ("droppedOctetDeltaCount", be_int),
        133 => ("droppedPacketDeltaCount", be_int),
        134 => ("droppedOctetTotalCount", be_int),
        135 => ("droppedPacketTotalCount", be_int),
        136 => ("flowEndReason", be_int),
        137 => ("classOfServiceIPv6", be_int),
        138 => ("postClassOfServiceIPv6", be_int),
        139 => ("icmpTypeCodeIPv6", be_int),
        140 => ("mplsTopLabelIPv6Address", ipv6_addr),
        141 => ("lineCardId", be_int),
        142 => ("portId", be_int),
        143 => ("meteringProcessId", be_int),
        144 => ("exportingProcessId", be_int),
        145 => ("templateId", be_int),
        146 => ("wlanChannelId", be_int),
        147 => ("wlanSsid", be_int),
        148 => ("flowId", be_int),
        149 => ("sourceId", be_int),
        150 => ("flowStartSeconds", be_int),
        151 => ("flowEndSeconds", be_int),
        152 => ("flowStartMilliSeconds", be_int),
        153 => ("flowEndMilliSeconds", be_int),
        154 => ("flowStartMicroSeconds", be_int),
        155 => ("flowEndMicroSeconds", be_int),
        156 => ("flowStartNanoSeconds", be_int),
        157 => ("flowEndNanoSeconds", be_int),
        158 => ("flowStartDeltaMicroSeconds", be_int),
        159 => ("flowEndDeltaMicroSeconds", be_int),
        160 => ("systemInitTimeMilliSeconds", be_int),
        161 => ("flowDurationMilliSeconds", be_int),
        162 => ("flowDurationMicroSeconds", be_int),
        163 => ("observedFlowTotalCount", be_int),
        164 => ("ignoredPacketTotalCount", be_int),
        165 => ("ignoredOctetTotalCount", be_int),
        166 => ("notSentFlowTotalCount", be_int),
        167 => ("notSentPacketTotalCount", be_int),
        168 => ("notSentOctetTotalCount", be_int),
        169 => ("destinationIPv6Prefix", be_int),
        170 => ("sourceIPv6Prefix", be_int),
        171 => ("postOctetTotalCount", be_int),
        172 => ("postPacketTotalCount", be_int),
        173 => ("flowKeyIndicator", be_int),
        174 => ("postMCastPacketTotalCount", be_int),
        175 => ("postMCastOctetTotalCount", be_int),
        176 => ("icmpTypeIPv4", be_int),
        177 => ("icmpCodeIPv4", be_int),
        178 => ("icmpTypeIPv6", be_int),
        179 => ("icmpCodeIPv6", be_int),
        180 => ("udpSourcePort", be_int),
        181 => ("udpDestinationPort", be_int),
        182 => ("tcpSourcePort", be_int),
        183 => ("tcpDestinationPort", be_int),
        184 => ("tcpSequenceNumber", be_int),
        185 => ("tcpAcknowledgementNumber", be_int),
        186 => ("tcpWindowSize", be_int),
        187 => ("tcpUrgentPointer", be_int),
        188 => ("tcpHeaderLength", be_int),
        189 => ("ipHeaderLength", be_int),
        190 => ("totalLengthIPv4", be_int),
        191 => ("payloadLengthIPv6", be_int),
        192 => ("ipTimeToLive", be_int),
        193 => ("nextHeaderIPv6", be_int),
        194 => ("ipClassOfService", be_int),
        195 => ("ipDiffServCodePoint", be_int),
        196 => ("ipPrecedence", be_int),
        197 => ("fragmentFlagsIPv4", be_int),
        198 => ("octetDeltaSumOfSquares", be_int),
        199 => ("octetTotalSumOfSquares", be_int),
        200 => ("mplsTopLabelTtl", be_int),
        201 => ("mplsLabelStackLength", be_int),
        202 => ("mplsLabelStackDepth", be_int),
        203 => ("mplsTopLabelExp", be_int),
        204 => ("ipPayloadLength", be_int),
        205 => ("udpMessageLength", be_int),
        206 => ("isMulticast", be_int),
        207 => ("internetHeaderLengthIPv4", be_int),
        208 => ("ipv4Options", be_int),
        209 => ("tcpOptions", be_int),
        210 => ("paddingOctets", be_int),
        213 => ("headerLengthIPv4", be_int),
        214 => ("mplsPayloadLength", be_int)
    }
}
