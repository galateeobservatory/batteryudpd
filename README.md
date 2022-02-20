# BatteryUDPd

This soft receives UDP message from Steca Tarom 4545 and store it to postgresql database.

## Installation

To compile, simply launch `cargo build --release` in the root directory.

### How to use

First create a table in your database named **battery** with following columns:

| column                        | data type                   |
|:------------------------------|:----------------------------|
| id                            | integer                     |
| servdate                      | timestamp without time zone |
| num_ver                       | integer                     |
| jour_sommet                   | date                        |
| heure_sommet                  | time without time zone      |
| tension_bat                   | real                        |
| tension_pv1                   | real                        |
| tension_pv2                   | real                        |
| charge                        | real                        |
| test_capacite                 | real                        |
| courant_total_charge_decharge | real                        |
| courant_pv1                   | real                        |
| courant_pv2                   | real                        |
| courant_entree_appareil       | real                        |
| courant_charge_total          | real                        |
| courant_consommateur          | real                        |
| courant_decharge_total        | real                        |
| temperature                   | real                        |
| erreur                        | integer                     |
| mode_charge                   | character                   |
| etat_commut_sortie_charge     | boolean                     |
| etat_commut_aux1              | boolean                     |
| etat_commut_aux2              | boolean                     |
| entree_energie_24h            | real                        |
| entree_energie_total          | real                        |
| sortie_energie_24h            | real                        |
| sortie_energie_total          | real                        |
| reduction                     | boolean                     |
| crc16                         | character varying (>= 4)    |
| erreur_crc16                  | boolean                     |

Then create a **.env** file in the same directory than the executable with the following variables:

```
POSTGRES_HOST=[DB location (localhost?)]
POSTGRES_PORT=[DB port (5432?)]
POSTGRES_USER=[DB username]
POSTGRES_PASSWORD=[DB password]
POSTGRES_DB=[DB name]
LISTEN_BINDING=[Address and port to listen (example: 0.0.0.0:20000)]
```
