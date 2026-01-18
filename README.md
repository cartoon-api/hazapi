# hazapi
An API to retrieve data on characters, cities, and other categories from the Prime Video series Hazbin Hotel and Helluva Boss.

---

## Universes

* hazbin_hotel
* helluva_boss

---

## Species

* angel
* demon
* imp
* spider_demon
* cat_demon
* snake_demon
* moth_demon
* hellhound
* succubus

---

## Characters Endpoints

### GET /characters

Returns all available characters.

### GET /characters/{character_name}

Returns data for a specific character.

Example:

```
/characters/charlie_morningstar
```

### GET /characters/{universe}

Filters characters by universe.

Examples:

```
/characters/hazbin_hotel
/characters/helluva_boss
```

---

## Species Endpoints

### GET /species/{specie}

Returns all characters that belong to the specified species.

Example:

```
/species/demon
```

---

## Weapons Endpoints

### GET /weapons

Returns all registered weapons.

### GET /weapons/{weapon_name}

Returns data for a specific weapon.

Example:

```
/weapons/angelic_spear
```

### GET /weapons/{universe}

Filters weapons by universe.

Examples:

```
/weapons/hazbin_hotel
/weapons/helluva_boss
```

---

## Technologies

* Rust
* Axum
* SQLx

---

## Community

* Community GitHub: [https://github.com/cartoon-api/](https://github.com/cartoon-api/)
* Maintainer GitHub: [https://github.com/rafasoliv-linux](https://github.com/rafasoliv-linux)

---

## Credits

Maintained by Rafael Oliveira

Email: [rafaeldossantosoliveira@proton.me](mailto:rafaeldossantosoliveira@proton.me)

---

hazapi is a fan-made project. We're continuously improving the API and documentation.

