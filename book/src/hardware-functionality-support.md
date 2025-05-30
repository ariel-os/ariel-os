# Hardware & Functionality Support

> The tables below indicate whether we support using the piece of functionality in a portable manner, trough an abstraction layer and platform-aware configuration.

Hardware support is organized into tiers, each with their own testing policy:

1. Tier 1 hardware gets regularly tested, either automatically or manually.
2. Tier 2 hardware only gets tested infrequently, but Ariel OS maintainers do have access to the hardware.
3. Tier 3 hardware is build-tested only, as Ariel OS maintainers do not have access to the hardware.

Tiers therefore are not related to the functionality coverage of each piece of hardware, and only says something about how much testing they undergo.

## Tier 1

Tier 1 hardware gets regularly tested, either automatically or manually.

<!-- cmdrun ../../doc/gen_support_matrix_html.rs generate ../../doc/support_matrix.yml /dev/stdout --tier 1 -->

## Tier 2

Tier 2 hardware only gets tested infrequently, but Ariel OS maintainers do have access to the hardware.

<!-- cmdrun ../../doc/gen_support_matrix_html.rs generate ../../doc/support_matrix.yml /dev/stdout --tier 2 -->

## Tier 3

Tier 3 hardware is build-tested only, as Ariel OS maintainers do not have access to the hardware.

<!-- cmdrun ../../doc/gen_support_matrix_html.rs generate ../../doc/support_matrix.yml /dev/stdout --tier 3 -->
