# Statshammer API

Contains the REST API and core logic for calculating statistical damage outputs for various Warhammer products (sometimes also referred to as Mathhammer).

## Supported Games

- Age of Sigmar
- Warcry _[coming soon]_

## Related Projects

- [AoS Statshammer](https://github.com/damonhook/aos-statshammer)
  - React based frontend for interacting with the calculated statistics for Warhammer Age of Sigmar units.
- [Warcry Statshammer](https://github.com/damonhook/warcry-statshammer)
  - React based frontend for interacting with the calculated statistics for Warhammer Warcry fighters.

## Crates

This project is organised into smaller `crates` to handle specific parts of the process, following a 3 layer design of _"Interface > Aggregate > Calculate"_.

- `statshammer-api` _[coming soon]_
  - Contains the REST based API for calculating the damage statistics for the supported games. This is a very light interface layer, with the logic being called from `aos-statshammer` and `warcry-statshammer` crates.
- `aos-statshammer`
  - Contains the aggregation logic for comparing the statistics for multiple Age of Sigmar units (using calculations performed by `aos-statshammer-core`).
- `aos-statshammer-core`
  - Contains the core logic for calculating the various statistics for a specific Age of Sigmar weapon (and associated abilities).
- `warcry-statshammer`
  - Contains the aggregation logic for comparing the statistics for multiple Warcry fighters (using calculations performed by `warcrys-statshammer-core`).
- `warcry-statshammer-core` _[coming soon]_
  - Contains the core logic for calculating the various statistics for a specific Warcry weapon.

### Architecture

```mermaid
graph TD
  statshammer-api --> aos-statshammer
  statshammer-api --> warcry-statshammer
  subgraph Age of Sigmar
    aos-statshammer --> aos-statshammer-core
  end
  subgraph Warcry
    warcry-statshammer --> warcry-statshammer-core
  end
```

### Example Flow

This is an **extremely simplified** example flow to illustrate how the pieces fit together.

```mermaid
sequenceDiagram
  participant User
  participant API as aos-statshammer-api
  participant Aggregator as aos-statshammer
  participant Core as aos-statshammer-core

  User ->>+ API: REST based API call
  API ->>+ API: Handles request data
  deactivate API
  API ->>+ Aggregator: Calls comparator with units
  loop unit in units
    loop weapon for unit
      Aggregator ->>+ Core: Calls processors for weapon
      Core -->>- Aggregator: Returns average damage results
    end
    loop weapon for unit
      Aggregator ->>+ Core: Calls max processor for weapon
      Core -->>- Aggregator: Returns max damage results
    end
  end
  Aggregator ->>+ Aggregator: Create comparison results
  deactivate Aggregator
  Aggregator -->>- API: Return comparison results
  API -->>- User: Return Results as JSON
```

## Disclaimer

This tool is in no way endorsed or sanctioned by Games Workshop - it is unnoffical and fan-made.
