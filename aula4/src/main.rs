// src/lib.rs

// Função que multiplica os elementos de um array usando ponteiro
pub unsafe fn multiply_array(ptr: *const i32, len: usize) -> i32 {
    let mut product = 1;
    for i in 0..len { // Começar em 0 para incluir o primeiro elemento
        product *= *ptr.add(i); // Usar `add` ao invés de `offset`
    }
    product
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiply_array() {
        let arr = [2, 3, 4];
        let product = unsafe { multiply_array(arr.as_ptr(), arr.len()) };
        assert_eq!(product, 24); // Verifica se a multiplicação é 24 (2 * 3 * 4)
    }
}

fn main() {
    // Exemplo para testar a função multiply_array
    let arr = [2, 3, 4];
    let product = unsafe { multiply_array(arr.as_ptr(), arr.len()) };
    println!("Produto dos elementos: {}", product); // Exibe o produto
}
