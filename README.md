# DES-queue

Simulatore a eventi discreti (DES) di una coda **M/M/c** scritto in Rust, con interfaccia grafica
(egui) che confronta in tempo reale i risultati della simulazione con i valori teorici.

## Requisiti

- [Rust](https://www.rust-lang.org/tools/install) con supporto all'edition 2024 (Rust ≥ 1.85)
- Un ambiente grafico (X11 o Wayland su Linux, oppure Windows/macOS) per la finestra di egui

## Avvio

```bash
cargo run --release
```

La modalità `--release` è consigliata: la simulazione è molto più veloce.

## Configurazione

I parametri del sistema sono costanti definite in `src/main.rs`:

```rust
const LAMBDA: f64 = 767.0; // tasso di arrivo (λ)
const MU: f64 = 81.0;      // tasso di servizio di ciascun server (μ)
const C: usize = 24;       // numero di server (c)
```

Modifica i valori e ricompila. Il programma verifica all'avvio che i parametri siano positivi
e che il sistema sia stabile, cioè `ρ = λ / (c·μ) < 1`; in caso contrario termina con un errore.

## Interfaccia

La finestra mostra:

- **Tabella delle statistiche**: `L_q`, `L_x`, `L_s`, `W_q`, `W_s` e `P_queue` (probabilità di
  attesa, formula C di Erlang), con valore simulato, valore teorico ed errore relativo percentuale.
- **Totali**: numero di arrivi e partenze elaborati.
- **Distribuzione dei tempi di interarrivo** e **dei tempi di servizio**: istogrammi confrontati
  con le densità esponenziali teoriche.
- **Distribuzione P_k**: probabilità di avere `k` utenti nel sistema, simulata e teorica.

La simulazione è volutamente rallentata (circa un evento ogni 10 ms) per poterne seguire
l'evoluzione. Il pulsante **"Fast-forward 200k events"** elabora 200.000 eventi alla massima
velocità, così le statistiche convergono rapidamente verso i valori teorici.

## Struttura del progetto

```
src/
├── main.rs              parametri (λ, μ, c) e avvio
├── utils.rs             formule teoriche (Erlang B/C, P_k, L, W)
├── simulation/          motore DES: eventi, pacchetti, nodo, simulatore
├── statistics/          raccolta delle statistiche durante la simulazione
└── plots/               interfaccia grafica e grafici (egui / egui_plot)
```
