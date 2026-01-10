# Traits & Generics (capture_utils)
Quick read: Extension points and the constraints they impose.

## Extensibility traits
- None exported; primary interfaces come from `vision_core::Recorder` which this crate implements.

## Glue traits
- Internal helper trait `GetPixelChecked` (sealed to this module) to avoid panics when drawing rectangles. Provides `get_pixel_mut_checked(&mut self, x, y) -> Option<&mut Rgba<u8>>`.

## Generics and bounds
- `JsonRecorder` implements `vision_core::Recorder` as a trait object friendly sink; no generics.
- No generic data structures; functions operate on concrete types (`Path`, `RgbaImage`, etc.).

## Design notes
- Intentional lack of generics keeps recorder/overlay utilities straightforward and trait-object compatible.
- If additional recorder types are added, implement `vision_core::Recorder` rather than introducing new traits to keep consumers unified.

## Links
- Source: `crates/capture_utils/src/lib.rs`
