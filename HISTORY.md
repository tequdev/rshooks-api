# Release History

## Unreleased


## 0.6.0 (2024-10-07)

* Added `require` and `is_buffer_equal_20`, `is_buffer_equal_32` helpers
* Added `drops_to_amount` helper
* Revert strict types
* Added optional fn to Result
* Added `slice` and `slice_mut` helpers

## 0.5.0 (2024-9-22)

* Refactor API functions to use specific types
* Refactor amount buffer types for native and non-native amounts
  * Including Breaking Changes

## 0.4.0 (2024-9-20)

* Update for latest Hooks

## 0.3.1 (2021-10-08)

* Fixed guard violation in some cases

## 0.3.0 (2021-10-08)

* Changed FieldId enum repr to `#[repr(u32)]`
* Improved `slot_type` API
* Added errors enum
* Changed Result type with new errors enum
* Changed enum representations

## 0.2.0 (2021-10-05)

* Added slot API
* Added util_keylet() API
* Added float API
* Changed modules layout
* Added concise docs

## 0.1.0 (2021-10-03)

* Initial release
