pub fn main() {
    println!("\n=== Memoria: Stack vs Heap ===\n");

    println!("Stack:");
    println!("  • Almacena valores conocidos en tiempo de compilación.");
    println!("  • Acceso rápido (LIFO: último en entrar, primero en salir).");
    println!("  • Memoria contigua, tamaño fijo (definido en compilación).");
    println!("  • Menos flexible, pero más eficiente en acceso.\n");

    println!("Heap:");
    println!("  • Almacena datos asignados dinámicamente.");
    println!("  • Menos organizado: el SO asigna un bloque y devuelve un puntero.");
    println!("  • Se debe seguir el puntero para acceder a los datos.");
    println!("  • La memoria se libera automáticamente al salir del scope.\n");

    println!("=== Ejemplos ===\n");

    // Ejemplo con enteros (stack)
    let x: i32 = 5;
    let y: i32 = x; //.clone(); // Para enteros es barato clonar (se copia el valor)
    println!(
        "Enteros (Stack) - Para enteros es barato clonar (se copia el valor) - Pero enteros, booleanos y otros elementos de pila tienen ya el rasgo Copy que auto copia el contenido:"
    );
    println!("  x = {}, y = {} (clon de x)", x, y);

    // Ejemplo con String (heap)
    let s1: String = String::from("hello");
    let s2: &String = &s1; // Se crea una referencia, ya que copiar un String es costoso
    println!("\nStrings (Heap) - Se crea una referencia, ya que copiar un String es costoso:");
    println!("  s1 = \"{}\"", s1);
    println!("  &s1 == s2 // s1 ==  *s2 == \"{}\" (referencia a s1)", s2);

    println!("\nReference vs Dereference:");
    println!(
        "  • Reference (referencia): es un puntero que apunta a un valor, por ejemplo, s2 es una REFERENCIA a s1 con &s1."
    );
    println!(
        "  • Dereference (desreferencia): se usa el operador * para acceder al valor al que apunta la referencia (por ejemplo, *s2 accede al CONTENIDO de s1)."
    );

    println!("\nTraits de impresión:");
    println!("  • Display: imprime el valor referenciado.");
    println!("  • Debug: imprime la representación de la variable o referencia.");
    println!("---\n")
}
