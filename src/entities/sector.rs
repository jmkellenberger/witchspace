use std::fmt::Display;

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct Sector {
    pub seed: Seed,
    pub systems: Vec<System>,
}

impl Display for Sector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Widths
        let hex: String = format!("{:4}", "Hex");
        let hex_dash: String = "-".repeat(4);
        let uwp: String = format!("{:9}", "UWP");
        let uwp_dash: String = "-".repeat(9);
        let ix: String = format!("{:6}", "{Ix}");
        let ix_dash: String = "-".repeat(6);
        let ex: String = format!("{:7}", "{Ex}");
        let ex_dash: String = "-".repeat(7);
        let cx: String = format!("{:6}", "{Cx}");
        let cx_dash: String = "-".repeat(6);
        let bases: String = format!("{:2}", "B");
        let bases_dash: String = "-".repeat(2);
        let zone: String = format!("{:1}", "Z");
        let zone_dash: String = "-".repeat(1);
        let pbg: String = format!("{:3}", "PBG");
        let pbg_dash: String = "-".repeat(3);
        let worlds: String = format!("{:2}", "W");
        let worlds_dash: String = "-".repeat(2);
        let allegiance: String = format!("{:4}", "A");
        let allegiance_dash: String = "-".repeat(4);

        let name_width = self
            .systems
            .iter()
            .map(|sys| sys.name.len())
            .reduce(|acc, x| if x > acc { x } else { acc })
            .unwrap();

        let name = format!("{:width$}", "Name", width = name_width);
        let name_dash = "-".repeat(name_width);

        let remarks_width = self
            .systems
            .iter()
            .map(|sys| sys.trade_codes().len())
            .reduce(|acc, x| if x > acc { x } else { acc })
            .unwrap();

        let remarks = format!("{:width$}", "Remarks", width = remarks_width);
        let remarks_dash = "-".repeat(remarks_width);

        let nobility_width = self
            .systems
            .iter()
            .map(|sys| sys.nobility().len())
            .reduce(|acc, x| if x > acc { x } else { acc })
            .unwrap();

        let nobility = format!("{:width$}", "N", width = nobility_width);
        let nobility_dash = "-".repeat(nobility_width);

        let stellar_width = self
            .systems
            .iter()
            .map(|sys| sys.stars.to_string().len())
            .reduce(|acc, x| if x > acc { x } else { acc })
            .unwrap();

        let stellar = format!("{:width$}", "Stellar", width = stellar_width);
        let stellar_dash = "-".repeat(stellar_width);
        // Hex  Name                 UWP       Remarks                                  {Ix}   (Ex)    [Cx]   N     B  Z PBG W  A    Stellar
        // ---- -------------------- --------- ---------------------------------------- ------ ------- ------ ----- -- - --- -- ---- --------------
        let labels = format!(
            "{} {} {} {} {} {} {} {} {} {} {} {} {} {} ",
            hex,
            name,
            uwp,
            remarks,
            ix,
            ex,
            cx,
            nobility,
            bases,
            zone,
            pbg,
            worlds,
            allegiance,
            stellar
        );

        let separator = format!(
            "{} {} {} {} {} {} {} {} {} {} {} {} {} {} ",
            hex_dash,
            name_dash,
            uwp_dash,
            remarks_dash,
            ix_dash,
            ex_dash,
            cx_dash,
            nobility_dash,
            bases_dash,
            zone_dash,
            pbg_dash,
            worlds_dash,
            allegiance_dash,
            stellar_dash
        );

        write!(
            f,
            "{}\n{}",
            format!("{}\n{}", labels, separator),
            self.systems
                .iter()
                .map(|sys| format!(
                    "{} {:name_width$} {:10} {:remarks_width$} {} {:nobility_width$} {:2} {:1} {:3} {:2} {:4} {:stellar_width$}",
                    sys.location,
                    sys.name,
                    sys.mainworld,
                    sys.trade_codes(),
                    sys.extensions,
                    sys.nobility(),
                    sys.bases(),
                    sys.travel_zone(),
                    sys.pbg(),
                    sys.worlds,
                    sys.allegiance,
                    sys.stars
                ))
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}
