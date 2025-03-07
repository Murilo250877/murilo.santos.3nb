use std::boxed::Box;
use std::ptr;

pub struct Element<T> {
    value: T,
    next: Option<Box<Element<T>>>,
}

pub struct Queue<T> {
    first: Option<Box<Element<T>>>,
    last: *mut Option<Box<Element<T>>>,
    count: usize,
}

impl<T> Queue<T> {
    // Cria uma fila vazia
    pub fn new() -> Self {
        Queue {
            first: None,
            last: ptr::null_mut(),
            count: 0,
        }
    }

    // Insere um item no final da fila
    pub fn push(&mut self, item: T) {
        let new_item = Box::new(Element {
            value: item,
            next: None,
        });

        // Fila vazia
        if self.count == 0 {
            self.first = Some(new_item);
            self.last = unsafe { &mut self.first.as_mut().unwrap().next as *mut _ };
        } else {
            // Fila não vazia
            unsafe {
                (*self.last).replace(new_item);
            }
            // Atualiza a cauda
            self.last = unsafe { &mut (*self.last).as_mut().unwrap().next as *mut _ };
        }

        self.count += 1;
    }

    // Remove e retorna o primeiro item da fila
    pub fn pop(&mut self) -> Option<T> {
        if let Some(first_item) = self.first.take() {
            self.first = first_item.next;
            if self.first.is_none() {
                self.last = ptr::null_mut();
            }
            self.count -= 1;
            Some(first_item.value)
        } else {
            None
        }
    }

    // Olha o primeiro item da fila sem removê-lo
    pub fn front(&self) -> Option<&T> {
        self.first.as_ref().map(|item| &item.value)
    }

    // Retorna a quantidade de itens na fila
    pub fn size(&self) -> usize {
        self.count
    }

    // Verifica se a fila está vazia
    pub fn is_empty(&self) -> bool {
        self.count == 0
    }
}

impl<T> Drop for Queue<T> {
    fn drop(&mut self) {
        while let Some(_) = self.pop() {} // Libera a memória
    }
}

fn main() {
    let mut queue = Queue::new();

    // Simulando a chegada de clientes na fila
    queue.push("Cliente 1");
    queue.push("Cliente 2");
    queue.push("Cliente 3");

    println!("Fila inicial:");
    println!("Primeiro cliente na fila: {:?}", queue.front());

    // Atendimento dos clientes
    while !queue.is_empty() {
        if let Some(client) = queue.pop() {
            println!("Atendendo: {}", client);
        }
    }

    println!("Todos os clientes foram atendidos.");
}
