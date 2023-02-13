# Benchmarks

## Table of Contents

- [Benchmark Results](#benchmark-results)
    - [sample_edonbls12_381_optimized](#sample_edonbls12_381_optimized)
    - [arithmetic_for_edonbls12_381_optimized](#arithmetic_for_edonbls12_381_optimized)
    - [serialization_for_edonbls12_381_optimized](#serialization_for_edonbls12_381_optimized)
    - [msm_for_edonbls12_381_optimized](#msm_for_edonbls12_381_optimized)
    - [squareroot_for_edonbls12_381_optimized](#squareroot_for_edonbls12_381_optimized)
    - [bitwise_operations_for_edonbls12_381_optimized](#bitwise_operations_for_edonbls12_381_optimized)
    - [conversions_for_edonbls12_381_optimized](#conversions_for_edonbls12_381_optimized)

## Benchmark Results

### sample_edonbls12_381_optimized

|        | `goptimized_elements`           |
|:-------|:------------------------------- |
|        | `66.55 us` (✅ **1.00x**)        |

### arithmetic_for_edonbls12_381_optimized

|                                       | `froptimized::bigint`          | `fqoptimized::bigint`          | `goptimized`              | `fqoptimized`                   | `froptimized`                    |
|:--------------------------------------|:-------------------------------|:-------------------------------|:--------------------------|:--------------------------------|:-------------------------------- |
| **`addition`**                        | `N/A`                          | `N/A`                          | `388.29 ns` (✅ **1.00x**) | `8.72 ns` (🚀 **44.55x faster**) | `8.63 ns` (🚀 **45.01x faster**)  |
| **`subtraction`**                     | `N/A`                          | `N/A`                          | `407.79 ns` (✅ **1.00x**) | `8.81 ns` (🚀 **46.26x faster**) | `8.79 ns` (🚀 **46.37x faster**)  |
| **`mixed_addition`**                  | `N/A`                          | `N/A`                          | `398.69 ns` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`mixed_subtraction`**               | `N/A`                          | `N/A`                          | `407.75 ns` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`double`**                          | `N/A`                          | `N/A`                          | `295.09 ns` (✅ **1.00x**) | `5.88 ns` (🚀 **50.21x faster**) | `5.86 ns` (🚀 **50.36x faster**)  |
| **`scalar_multiplication`**           | `N/A`                          | `N/A`                          | `146.30 us` (✅ **1.00x**) | `N/A`                           | `N/A`                            |
| **`negation`**                        | `N/A`                          | `N/A`                          | `N/A`                     | `6.13 ns` (✅ **1.01x faster**)  | `6.16 ns` (✅ **1.00x**)          |
| **`multiplication`**                  | `N/A`                          | `N/A`                          | `N/A`                     | `42.90 ns` (✅ **1.00x slower**) | `42.79 ns` (✅ **1.00x**)         |
| **`square`**                          | `N/A`                          | `N/A`                          | `N/A`                     | `35.39 ns` (✅ **1.01x slower**) | `35.05 ns` (✅ **1.00x**)         |
| **`inverse`**                         | `N/A`                          | `N/A`                          | `N/A`                     | `6.87 us` (✅ **1.02x faster**)  | `7.01 us` (✅ **1.00x**)          |
| **`sum_of_products_of_size_2`**       | `N/A`                          | `N/A`                          | `N/A`                     | `61.53 ns` (✅ **1.00x faster**) | `61.56 ns` (✅ **1.00x**)         |
| **`naive_sum_of_products_of_size_2`** | `N/A`                          | `N/A`                          | `N/A`                     | `88.97 ns` (✅ **1.01x faster**) | `89.82 ns` (✅ **1.00x**)         |
| **`addition_with_carry`**             | `7.61 ns` (✅ **1.00x**)        | `7.61 ns` (✅ **1.00x slower**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`subtraction_with_borrow`**         | `8.74 ns` (✅ **1.00x**)        | `8.75 ns` (✅ **1.00x slower**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`multiplication_by_2`**             | `4.69 ns` (✅ **1.00x**)        | `4.69 ns` (✅ **1.00x faster**) | `N/A`                     | `N/A`                           | `N/A`                            |
| **`division_by_2`**                   | `4.53 ns` (✅ **1.00x**)        | `4.54 ns` (✅ **1.00x slower**) | `N/A`                     | `N/A`                           | `N/A`                            |

### serialization_for_edonbls12_381_optimized

|                                          | `goptimized`              | `froptimized`                      | `fqoptimized`                       |
|:-----------------------------------------|:--------------------------|:-----------------------------------|:----------------------------------- |
| **`serialize_compressed`**               | `68.16 ns` (✅ **1.00x**)  | `31.03 ns` (🚀 **2.20x faster**)    | `30.77 ns` (🚀 **2.21x faster**)     |
| **`serialize_uncompressed`**             | `62.30 ns` (✅ **1.00x**)  | `30.38 ns` (🚀 **2.05x faster**)    | `30.80 ns` (🚀 **2.02x faster**)     |
| **`deserialize_compressed`**             | `181.62 us` (✅ **1.00x**) | `49.82 ns` (🚀 **3645.72x faster**) | `52.26 ns` (🚀 **3475.54x faster**)  |
| **`deserialize_compressed_unchecked`**   | `38.44 us` (✅ **1.00x**)  | `49.76 ns` (🚀 **772.42x faster**)  | `52.23 ns` (🚀 **735.84x faster**)   |
| **`deserialize_uncompressed`**           | `143.11 us` (✅ **1.00x**) | `49.65 ns` (🚀 **2882.09x faster**) | `52.15 ns` (🚀 **2744.12x faster**)  |
| **`deserialize_uncompressed_unchecked`** | `163.76 ns` (✅ **1.00x**) | `49.76 ns` (🚀 **3.29x faster**)    | `52.19 ns` (🚀 **3.14x faster**)     |

### msm_for_edonbls12_381_optimized

|        | `goptimized`            |
|:-------|:----------------------- |
|        | `1.31 s` (✅ **1.00x**)  |

### squareroot_for_edonbls12_381_optimized

|                          | `froptimized`            | `fqoptimized`                    |
|:-------------------------|:-------------------------|:-------------------------------- |
| **`square_root_for_qr`** | `12.13 us` (✅ **1.00x**) | `31.19 us` (❌ *2.57x slower*)    |
| **`legendre_for_qr`**    | `12.27 us` (✅ **1.00x**) | `10.90 us` (✅ **1.13x faster**)  |

### bitwise_operations_for_edonbls12_381_optimized

|                               | `froptimized::bigint`          | `fqoptimized::bigint`            |
|:------------------------------|:-------------------------------|:-------------------------------- |
| **`number_of_bits`**          | `4.85 ns` (✅ **1.00x**)        | `4.85 ns` (✅ **1.00x slower**)   |
| **`from_little-endian_bits`** | `48.79 ns` (✅ **1.00x**)       | `48.65 ns` (✅ **1.00x faster**)  |
| **`from_big-endian_bits`**    | `48.74 ns` (✅ **1.00x**)       | `48.56 ns` (✅ **1.00x faster**)  |
| **`comparison`**              | `4.89 ns` (✅ **1.00x**)        | `4.88 ns` (✅ **1.00x faster**)   |
| **`equality`**                | `5.42 ns` (✅ **1.00x**)        | `5.44 ns` (✅ **1.00x slower**)   |
| **`is_zero`**                 | `4.90 ns` (✅ **1.00x**)        | `4.90 ns` (✅ **1.00x faster**)   |

### conversions_for_edonbls12_381_optimized

|                   | `froptimized`            | `fqoptimized`                    |
|:------------------|:-------------------------|:-------------------------------- |
| **`from_bigint`** | `40.87 ns` (✅ **1.00x**) | `40.75 ns` (✅ **1.00x faster**)  |
| **`into_bigint`** | `23.74 ns` (✅ **1.00x**) | `22.50 ns` (✅ **1.06x faster**)  |

---
Made with [criterion-table](https://github.com/nu11ptr/criterion-table)

