# AIS BinaryBroadcastMessage parser

Ships periodically broadcast their positions and other data using
small packets over VHF using a protocol called AIS.

These packets can be easily received and decoded, for example using a
VHF receiver and `gnuais`, or with an RTL-SDR dongle and `rtl_ais`.

The resulting datagrams can be decoded using many programs, and for
Rust we have the `ais` crate.

The AIS protocol was designed to be somewhat extensible and allows
application-specific data to be broadcast using
"BinaryBroadcastMessage" packet types.

Besides their binary payload, these packets messages have so-called
DAC and FID fields in their header, which allows the sub-protocol to
be identified.

Unfortunately, the current `ais` crate does not decode these.  On the
other hand, it seems that the BBM messages are poorly standardized and
basically a huge mess.  (Not my opinion, but what I've read.)

This crate implements a `BinaryBroadcastMessage` decoder, currently
only for the "Environmental" packets (DAC 1, FID 26), such as those
transmitted by the "air gap" (i.e. sea level height) sensor placed at
the Lion's Gate bridge.

## Usage

The file `data/example1.txt` contains an example with one BBM message
(transmitted as two packets):
```
!AIVDM,2,1,2,A,8@30ojh0FQ1Si6HBNLW0>2`;111006?4Mk53F73Og;rBP02aPSAP0<D6,0*31
!AIVDM,2,2,2,A,001Sh00000,4*2A
```

You can decode it as follows:
```
% cargo run -- data/example1.txt
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/ais-bbm-tool data/example1.txt`
ENV Environmental {
    reports: [
        SensorReport {
            report_type: 1,
            day: 0,
            hour: 24,
            minute: 60,
            site_id: 35,
            sensor_data: StationId(
                StationId {
                    name: "LIONS GATE    ",
                },
            ),
        },
        SensorReport {
            report_type: 0,
            day: 0,
            hour: 24,
            minute: 60,
            site_id: 35,
            sensor_data: SiteLocation(
                SiteLocation {
                    longitude: -123.1387,
                    latitude: 49.31539,
                    altitude: NaN,
                    owner: 5,
                    data_timeout: 0,
                },
            ),
        },
        SensorReport {
            report_type: 10,
            day: 19,
            hour: 0,
            minute: 35,
            site_id: 35,
            sensor_data: AirGap(
                AirGap {
                    air_draught: NaN,
                    air_gap: 63.04,
                    air_gap_trend: 3,
                    fc_air_gap: NaN,
                    fc_day: 0,
                    fc_hour: 24,
                    fc_minute: 60,
                },
            ),
        },
    ],
}
```

Author: Berke DURAK <bd@exhrd.fr>
