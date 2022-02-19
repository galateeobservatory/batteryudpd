use crate::config::Config;
use crate::battery_reading_line::BatteryReadingLine;
use postgres::{Client, NoTls};
use std::cell::RefCell;

/// A struct to hold the battery database connection
#[allow(dead_code)]
pub struct BatteryDatabase {
    database_client: RefCell<Client>,
}

#[allow(dead_code)]
impl BatteryDatabase {
    pub fn new(config: &Config) -> Result<Self, &'static str> {
        Ok(Self {
            database_client: RefCell::new(
                Client::configure()
                    .host(&*config.db_host)
                    .port(config.db_port)
                    .user(&*config.db_user)
                    .password(&*config.db_password)
                    .dbname(&*config.db_name)
                    .connect(NoTls)
                    .map_err(|_| "BatteryDatabase: Failed to connect to database")?,
            ),
        })
    }

    // INSERT INTO battery (num_ver, jour_sommet, heure_sommet, tension_bat, tension_pv1, tension_pv2, charge, test_capacite, courant_total_charge_decharge, courant_pv1, courant_pv2, courant_entree_appareil, courant_charge_total, courant_consommateur, courant_decharge_total, temperature, erreur, mode_charge, etat_commut_sortie_charge, etat_commut_aux1, etat_commut_aux2, entree_energie_24h, entree_energie_total, sortie_energie_24h, sortie_energie_total, reduction, crc16, erreur_crc16)
    pub fn insert_line(&self, line: &BatteryReadingLine) -> Result<(), String> {
        let num_ver = match line.num_ver {
            Some( num_ver) => Some(num_ver as i32),
            None => None
        };
        let erreur = match line.erreur {
            Some( erreur) => Some(erreur as i32),
            None => None,
        };
        let mode_charge = match line.mode_charge {
            Some(mode) => Some(mode.to_string()),
            None => None,
        };
        let crc_16 = match line.crc16 {
            Some(crc16) => Some(format!("{:X}", crc16)),
            None => None,
        };
        self.database_client.borrow_mut().execute("INSERT INTO battery (num_ver, jour_sommet, heure_sommet, tension_bat, tension_pv1, tension_pv2, charge, test_capacite, courant_total_charge_decharge, courant_pv1, courant_pv2, courant_entree_appareil, courant_charge_total, courant_consommateur, courant_decharge_total, temperature, erreur, mode_charge, etat_commut_sortie_charge, etat_commut_aux1, etat_commut_aux2, entree_energie_24h, entree_energie_total, sortie_energie_24h, sortie_energie_total, reduction, crc16, erreur_crc16) VALUES ($1, $2 ,$3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23, $24, $25, $26, $27, $28)",
                                                  &[
                                                      &num_ver,
                                                      &line.jour_sommet,
                                                      &line.heure_sommet,
                                                      &line.tension_bat,
                                                      &line.tension_pv1,
                                                      &line.tension_pv2,
                                                      &line.charge,
                                                      &line.test_capacite,
                                                      &line.courant_total_charge_decharge,
                                                      &line.courant_pv1,
                                                      &line.courant_pv2,
                                                      &line.courant_entree_appareil,
                                                      &line.courant_charge_total,
                                                      &line.courant_consommateur,
                                                      &line.courant_decharge_total,
                                                      &line.temperature,
                                                      &erreur,
                                                      &mode_charge,
                                                      &line.etat_commut_sortie_charge,
                                                      &line.etat_commut_aux1,
                                                      &line.etat_commut_aux2,
                                                      &line.entree_energie_24h,
                                                      &line.entree_energie_total,
                                                      &line.sortie_energie_24h,
                                                      &line.sortie_energie_total,
                                                      &line.reduction,
                                                      &crc_16,
                                                      &line.erreur_crc16])
            .map_err(|e| format!("BatteryDatabase: Failed to insert line {}", e))?;
        Ok(())

    }
}
