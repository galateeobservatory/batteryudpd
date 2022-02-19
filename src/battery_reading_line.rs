use std::fmt::Debug;
use std::str::FromStr;
use crate::crc16_tarom4545;
use chrono::{NaiveDate, NaiveTime};

/// This struct represents a line sent by the Steca Tarom4545 battery monitor.
#[allow(dead_code)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct BatteryReadingLine {
    pub num_ver: Option<u8>,
    pub jour_sommet: Option<NaiveDate>,
    pub heure_sommet: Option<NaiveTime>,
    pub tension_bat: Option<f32>,
    pub tension_pv1: Option<f32>,
    pub tension_pv2: Option<f32>,
    pub charge: Option<f32>,
    pub test_capacite: Option<f32>,
    pub courant_total_charge_decharge: Option<f32>,
    pub courant_pv1: Option<f32>,
    pub courant_pv2: Option<f32>,
    pub courant_entree_appareil: Option<f32>,
    pub courant_charge_total: Option<f32>,
    pub courant_consommateur: Option<f32>,
    pub courant_decharge_total: Option<f32>,
    pub temperature: Option<f32>,
    pub erreur: Option<u8>,
    pub mode_charge: Option<char>,
    pub etat_commut_sortie_charge: Option<bool>,
    pub etat_commut_aux1: Option<bool>,
    pub etat_commut_aux2: Option<bool>,
    pub entree_energie_24h: Option<f32>,
    pub entree_energie_total: Option<f32>,
    pub sortie_energie_24h: Option<f32>,
    pub sortie_energie_total: Option<f32>,
    pub reduction: Option<bool>,
    pub crc16: Option<u16>,
    pub erreur_crc16: bool,
}

#[allow(dead_code)]
impl BatteryReadingLine {
    const BAD_LINE_VALUE: BatteryReadingLine = BatteryReadingLine {
        num_ver: None,
        jour_sommet: None,
        heure_sommet: None,
        tension_bat: None,
        tension_pv1: None,
        tension_pv2: None,
        charge: None,
        test_capacite: None,
        courant_total_charge_decharge: None,
        courant_pv1: None,
        courant_pv2: None,
        courant_entree_appareil: None,
        courant_charge_total: None,
        courant_consommateur: None,
        courant_decharge_total: None,
        temperature: None,
        erreur: None,
        mode_charge: None,
        etat_commut_sortie_charge: None,
        etat_commut_aux1: None,
        etat_commut_aux2: None,
        entree_energie_24h: None,
        entree_energie_total: None,
        sortie_energie_24h: None,
        sortie_energie_total: None,
        reduction: None,
        crc16: None,
        erreur_crc16: true,
    };

    pub fn new(battery_line: &str) -> Result<Self, &'static Self> {
        crc16_tarom4545::validate_line(battery_line).map_err(|_| &Self::BAD_LINE_VALUE)?;
        Ok(Self::extract_from_str(battery_line).map_err(|_| &Self::BAD_LINE_VALUE)?)
    }

    fn extract_from_str(battery_line: &str) -> Result<Self, ()> {
        let battery_line_split: Vec<&str> = battery_line.split(";").collect();
        if battery_line_split.len() != 27 {
            return Err(());
        }
        Ok(Self {
            num_ver: Self::convert_type(battery_line_split[0])?,
            jour_sommet: match battery_line_split[1] {
                "#" => None,
                _ => Some(NaiveDate::parse_from_str(battery_line_split[1], "%Y/%m/%d").map_err(|_| ())?),
            },
            heure_sommet: match battery_line_split[2] {
                "#" => None,
                _ => Some(NaiveTime::parse_from_str(battery_line_split[2], "%H:%M").map_err(|_| ())?),
            },
            tension_bat: Self::convert_type(battery_line_split[3])?,
            tension_pv1: Self::convert_type(battery_line_split[4])?,
            tension_pv2: Self::convert_type(battery_line_split[5])?,
            charge: Self::convert_type(battery_line_split[6])?,
            test_capacite: Self::convert_type(battery_line_split[7])?,
            courant_total_charge_decharge: Self::convert_type(battery_line_split[8])?,
            courant_pv1: Self::convert_type(battery_line_split[9])?,
            courant_pv2: Self::convert_type(battery_line_split[10])?,
            courant_entree_appareil: Self::convert_type(battery_line_split[11])?,
            courant_charge_total: Self::convert_type(battery_line_split[12])?,
            courant_consommateur: Self::convert_type(battery_line_split[13])?,
            courant_decharge_total: Self::convert_type(battery_line_split[14])?,
            temperature: Self::convert_type(battery_line_split[15])?,
            erreur: Self::convert_type(battery_line_split[16])?,
            mode_charge: Self::convert_type(battery_line_split[17])?,
            etat_commut_sortie_charge: Self::convert_type_bool(battery_line_split[18])?,
            etat_commut_aux1: Self::convert_type_bool(battery_line_split[19])?,
            etat_commut_aux2: Self::convert_type_bool(battery_line_split[20])?,
            entree_energie_24h: Self::convert_type(battery_line_split[21])?,
            entree_energie_total: Self::convert_type(battery_line_split[22])?,
            sortie_energie_24h: Self::convert_type(battery_line_split[23])?,
            sortie_energie_total: Self::convert_type(battery_line_split[24])?,
            reduction: Self::convert_type_bool(battery_line_split[25])?,
            crc16: Some(u16::from_str_radix(battery_line_split[26], 16).map_err(|_| ())?),
            erreur_crc16: false
        })
    }

    fn convert_type<T: 'static + FromStr + Debug>(field: &str) -> Result<Option<T>, ()>
    {
        match field {
            "#" => Ok(None),
            _ => {
                Ok(Some(field.parse().map_err(|_| ())?))
            }
        }
    }

    fn convert_type_bool(field: &str) -> Result<Option<bool>, ()>
    {
        match field {
            "#" => Ok(None),
            "1" | "T" => Ok(Some(true)),
            "0" | "F" => Ok(Some(false)),
            _ => Err(()),
        }
    }
}


#[cfg(test)]
mod test {
    use chrono::{NaiveDate, NaiveTime};
    use crate::battery_reading_line::BatteryReadingLine;

    #[test]
    fn test_line_ok() {
        let line_reading = "1;2021/03/01;23:26;26.2;1.1;#;98.0;#;-0.5;0.0;#;0.0;-0.5;0.5;0.5;3.3;0;F;1;0;0;15.0;10310.5;10.9;6662.4;0;F7BB";
        let battery_reading_line = BatteryReadingLine::new(line_reading).unwrap();
        assert_eq!(battery_reading_line.num_ver, Some(1));
        assert_eq!(battery_reading_line.jour_sommet, Some(NaiveDate::from_ymd(2021, 3, 1)));
        assert_eq!(battery_reading_line.heure_sommet, Some(NaiveTime::from_hms(23, 26, 0)));
        assert_eq!(battery_reading_line.tension_bat, Some(26.2));
        assert_eq!(battery_reading_line.tension_pv1, Some(1.1));
        assert_eq!(battery_reading_line.tension_pv2, None);
        assert_eq!(battery_reading_line.charge, Some(98.0));
        assert_eq!(battery_reading_line.test_capacite, None);
        assert_eq!(battery_reading_line.courant_total_charge_decharge, Some(-0.5));
        assert_eq!(battery_reading_line.courant_pv1, Some(0.0));
        assert_eq!(battery_reading_line.courant_pv2, None);
        assert_eq!(battery_reading_line.courant_entree_appareil, Some(0.0));
        assert_eq!(battery_reading_line.courant_charge_total, Some(-0.5));
        assert_eq!(battery_reading_line.courant_consommateur, Some(0.5));
        assert_eq!(battery_reading_line.courant_decharge_total, Some(0.5));
        assert_eq!(battery_reading_line.temperature, Some(3.3));
        assert_eq!(battery_reading_line.erreur, Some(0));
        assert_eq!(battery_reading_line.mode_charge, Some('F'));
        assert_eq!(battery_reading_line.etat_commut_sortie_charge, Some(true));
        assert_eq!(battery_reading_line.etat_commut_aux1, Some(false));
        assert_eq!(battery_reading_line.etat_commut_aux2, Some(false));
        assert_eq!(battery_reading_line.entree_energie_24h, Some(15.0));
        assert_eq!(battery_reading_line.entree_energie_total, Some(10310.5));
        assert_eq!(battery_reading_line.sortie_energie_24h, Some(10.9));
        assert_eq!(battery_reading_line.sortie_energie_total, Some(6662.4));
        assert_eq!(battery_reading_line.reduction, Some(false));
        assert_eq!(battery_reading_line.crc16, Some(0xF7BB));
        assert_eq!(battery_reading_line.erreur_crc16, false);
    }

    #[test]
    fn test_bad_crc() {
        let line_reading = "1;2021/03/01;23:26;26.2;1.1;#;98.0;#;-0.5;0.0;#;0.0;-0.5;0.5;0.5;3.3;0;F;1;0;0;15.0;10310.5;10.9;6662.4;0;F8BB";
        let result_battery_reading_line = BatteryReadingLine::new(line_reading);
        assert!(result_battery_reading_line.is_err());
        assert_eq!(result_battery_reading_line, Err(&BatteryReadingLine::BAD_LINE_VALUE));
    }
}