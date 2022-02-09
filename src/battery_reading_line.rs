use crate::crc16_tarom4545;
use chrono::{NaiveDate, NaiveTime};
use csv::{Reader, ReaderBuilder};

/// This struct represents a line sent by the Steca Tarom4545 battery monitor.
#[allow(dead_code)]
#[derive(Copy, Clone, Debug, Deserialize, PartialEq)]
struct BatteryReadingLine {
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

    pub fn new(battery_line: &str) -> Result<Self, Self> {
        crc16_tarom4545::validate_line(battery_line).map_err(|_| Self::BAD_LINE_VALUE)?;
        Self::extract_from_str(battery_line);
        Ok(Self::BAD_LINE_VALUE) // TODO
    }

    // extract from 1;2021/03/01;23:26;26.2;1.1;#;98.0;#;-0.5;0.0;#;0.0;-0.5;0.5;0.5;3.3;0;F;1;0;0;15.0;10310.5;10.9;6662.4;0;F7BB
    fn extract_from_str(battery_line: &str) -> Result<Self, ()> {
        let battery_line_split = battery_line.split(";");
        if battery_line_split.count() != 27 {
            return Err(());
        }

        let csv_line = String::from("num_ver;jour_sommet;heure_sommet;tension_bat;tension_pv1;tension_pv2;charge;test_capacite;courant_total_charge_decharge;courant_pv1;courant_pv2;\
        courant_entree_appareil;courant_charge_total;courant_consommateur;courant_decharge_total;temperature;erreur;mode_charge;etat_commut_sortie_charge;etat_commut_aux1;\
        etat_commut_aux2;entree_energie_24h;entree_energie_total;sortie_energie_24h;sortie_energie_total;reduction;crc16;erreur_crc16\n") + battery_line + ";true";
        println!("{}", csv_line);
        let mut rdr = ReaderBuilder::new().delimiter(b';').from_reader(csv_line.as_bytes());
        let mut iter = rdr.deserialize();
        if let Some(result) = iter.next() {
            let record: Self = result.map_err(|_| ())?;
            println!("{:?}", record);
            return Ok(record);
        }
        Err(())
    }
}


#[cfg(test)]
mod test {
    use crate::battery_reading_line::BatteryReadingLine;

    #[test]
    fn test_1() {
        println!("hello world");
        BatteryReadingLine::new("1;2021/03/01;23:26;26.2;1.1;#;98.0;#;-0.5;0.0;#;0.0;-0.5;0.5;0.5;3.3;0;F;1;0;0;15.0;10310.5;10.9;6662.4;0;F7BB");
    }
}