<details>
<summary>English translation</summary>

# finteredit

A small mod for Horizon (Minecraft: Java Edition 26.3, rewritten in Rust). Lets you select two points with left clicks and fill the resulting cuboid with one of the fixed blocks.

## Features

- `/wand` (or `//wand`) gives you a wooden axe and resets the current selection.
- With the axe in hand, left-click a block to set the first point, right-click to set the second. The block is not broken.
- The chat shows the point coordinates and the number of blocks in the selection (volume of the area) in dark orange.
- `//set <n>` fills the selected area with the chosen block. `/set` is not registered — only `//set` works.
- `//undo` (or `/undo`) reverts the last fill and restores the original blocks.

## Commands

<img src="https://img.shields.io/badge/version-0.1.6-7c5cff?style=for-the-badge" alt="version">
<img src="https://img.shields.io/badge/license-MIT-00c853?style=for-the-badge" alt="license">
<img src="https://img.shields.io/badge/Horizon-26.3-ff7a00?style=for-the-badge" alt="game">
<img src="https://img.shields.io/badge/built%20with-Rust-dea584?style=for-the-badge" alt="rust">

| Command | Action |
| --- | --- |
| `/wand` or `//wand` | Give a wooden axe and start a new selection |
| Left-click with the axe | Set the first point |
| Right-click with the axe | Set the second point |
| `//set <n>` | Fill the selection with block `n`. Double slash only |
| `//undo` or `/undo` | Undo the last fill |

## Available blocks

| `n` | Block |
| --- | --- |
| `0` | `minecraft:air` |
| `1` | `minecraft:stone` |
| `2` | `minecraft:grass_block` |
| `3` | `minecraft:dirt` |
| `4` | `minecraft:cobblestone` |
| `5:1` | `minecraft:oak_planks` |
| `5:2` | `minecraft:spruce_planks` |
| `5:3` | `minecraft:birch_planks` |
| `5:4` | `minecraft:jungle_planks` |
| `5:5` | `minecraft:acacia_planks` |
| `6` | `minecraft:bedrock` |

Example: `//set 5:3` — fill the selection with birch planks.

## Limitations

- `//set`, `//undo` and `/undo` require operator level 2.
- Maximum selection size: 32768 blocks.
- Undo history stores up to 8 recent actions and is cleared when the player leaves.
- Selection and undo are bound to the dimension.
- Undo restores blocks only; container contents are not restored.

## Localization

Mod messages are translated into Russian and English. Files live in `assets/finteredit/lang/`. The language is picked automatically from the client language. To add a new language, copy `en_us.json` under a new name and translate the values — see `LANGUAGES.md` for details.

## Project layout

```
finteredit/
├── horizon-mod.toml
├── Cargo.toml
├── Cargo.lock
├── mod.wasm
├── LICENSE
├── README.md
├── LANGUAGES.md
├── src/
│   ├── lib.rs
│   ├── blocks.rs
│   ├── commands.rs
│   ├── editor.rs
│   ├── i18n.rs
│   ├── interact.rs
│   └── selection.rs
├── assets/finteredit/lang/
│   ├── en_us.json
│   └── ru_ru.json
├── data/finteredit/
└── target/horizon-mod/finteredit.hmod
```

Each part of the mod lives in its own file: blocks, commands, filling, translations, clicks and selection.

## Building

You need Rust, `cargo-horizon-mod` and the SDK for the required Horizon build (pulled automatically).

```sh
cargo horizon-mod build --wasm
```

The package appears at `target/horizon-mod/finteredit.hmod`.

## Installation

1. Copy `finteredit.hmod` into the instance's `horizon-mods/` folder.
2. Add the line `mod.finteredit:true` to `horizon-mods.txt`.
3. Fully restart the instance.

The warning `wasm module cache disabled: ... Windows owner/DACL/inheritance trust is not verified` is safe: the on-disk cache is disabled, but the mod compiles and works.

## Author and license

Author: **sinvides**. License: **MIT** (see `LICENSE`).

Built for Horizon Nightly `2026-11-22` (`2b77287`), game 26.3. Version: **0.1.5**.

</details>

<details>
<summary>Русский перевод</summary>

# finteredit

Небольшой мод для Horizon (Minecraft: Java Edition 26.3, переписанный на Rust). Позволяет выделить две точки кликом ЛКМ и застроить получившийся параллелепипед одним из фиксированных блоков.

## Возможности

- Команда `/wand` (или `//wand`) выдаёт деревянный топор и сбрасывает текущее выделение.
- С топором в руке ЛКМ по блоку ставит первую точку, ПКМ — вторую. Блок при этом не ломается.
- В чат тёмно-оранжевым сообщением выводятся координаты точки и количество блоков в выделении (объём области).
- Команда `//set <n>` застраивает выделенную область выбранным блоком. Вариант `/set` не зарегистрирован — работает только `//set`.
- Команда `//undo` (или `/undo`) отменяет последнюю застройку и возвращает блоки в исходное состояние.

## Команды

<img src="https://img.shields.io/badge/version-0.1.6-7c5cff?style=for-the-badge" alt="version">
<img src="https://img.shields.io/badge/license-MIT-00c853?style=for-the-badge" alt="license">
<img src="https://img.shields.io/badge/Horizon-26.3-ff7a00?style=for-the-badge" alt="game">
<img src="https://img.shields.io/badge/built%20with-Rust-dea584?style=for-the-badge" alt="rust">

| Команда | Действие |
| --- | --- |
| `/wand` или `//wand` | Выдать деревянный топор и начать новое выделение |
| ЛКМ по блоку с топором | Поставить первую точку |
| ПКМ по блоку с топором | Поставить вторую точку |
| `//set <n>` | Заполнить выделенную область блоком под номером `n`. Только двойной слеш |
| `//undo` или `/undo` | Отменить последнюю застройку |

## Доступные блоки

`//set` принимает как числовой ID (легаси-таблица Minecraft 1.12), так и прямое имя блока.

| `n` | Блок |
| --- | --- |
| `0` | `minecraft:air` |
| `1` | `minecraft:stone` |
| `2` | `minecraft:grass_block` |
| `3` | `minecraft:dirt` |
| `4` | `minecraft:cobblestone` |
| `5:0` | `minecraft:oak_planks` |
| `5:1` | `minecraft:spruce_planks` |
| `5:2` | `minecraft:birch_planks` |
| `5:3` | `minecraft:jungle_planks` |
| `5:4` | `minecraft:acacia_planks` |
| `5:5` | `minecraft:dark_oak_planks` |
| `6` | `minecraft:oak_sapling` |
| `7` | `minecraft:bedrock` |

Примеры: `//set 7`, `//set 5:4`, `//set minecraft:air`, `//set stone`.

## Ограничения

- `//set`, `//undo` и `/undo` требуют уровень оператора 2.
- Максимальный размер выделения: 32768 блоков.
- История отмены хранит до 8 последних действий и очищается при выходе игрока.
- Выделение и отмена привязаны к измерению.
- При отмене восстанавливаются только блоки; содержимое сундуков не восстанавливается.

## Локализация

Сообщения мода переведены на русский и английский. Файлы лежат в `assets/finteredit/lang/`. Язык выбирается автоматически по языку клиента. Чтобы добавить новый язык, скопируйте `en_us.json` под новым именем и переведите значения — подробнее в `LANGUAGES.md`.

## Структура проекта

```
finteredit/
├── horizon-mod.toml
├── Cargo.toml
├── Cargo.lock
├── mod.wasm
├── LICENSE
├── README.md
├── LANGUAGES.md
├── src/
│   ├── lib.rs
│   ├── blocks.rs
│   ├── commands.rs
│   ├── editor.rs
│   ├── i18n.rs
│   ├── interact.rs
│   └── selection.rs
├── assets/finteredit/lang/
│   ├── en_us.json
│   └── ru_ru.json
├── data/finteredit/
└── target/horizon-mod/finteredit.hmod
```

Каждая часть мода лежит в отдельном файле: блоки, команды, застройка, переводы, клики и выделение.

## Сборка

Нужны Rust, `cargo-horizon-mod` и SDK нужной сборки Horizon (подтягивается автоматически).

```sh
cargo horizon-mod build --wasm
```

Готовый пакет появляется по пути `target/horizon-mod/finteredit.hmod`.

## Установка

1. Скопируйте `finteredit.hmod` в папку инстанса `horizon-mods/`.
2. В `horizon-mods.txt` добавьте строку `mod.finteredit:true`.
3. Полностью перезапустите инстанс.

Предупреждение `wasm module cache disabled: ... Windows owner/DACL/inheritance trust is not verified` безопасно: кэш на диске отключён, но мод компилируется и работает.

## Автор и лицензия

Автор: **sinvides**. Лицензия: **MIT** (файл `LICENSE`).

Мод собран под Horizon Nightly `2026-11-22` (`2b77287`), игра 26.3. Версия: **0.1.6**.

</details>
