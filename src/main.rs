// ============================================================
// Motor de Catálogo AVL - Biblioteca Municipal de Santa Ana
// Materia: Estructuras de Datos / Programación II
// ============================================================

// Clone permite copiar un Libro cuando sea necesario (ej: al extraer el sucesor).
// Debug permite imprimir la struct con {:?}
#[derive(Debug, Clone)]
struct Libro {
    isbn: u32,
    titulo: String,
}

// La estructura Nodo representa un nodo en el árbol AVL, que contiene un libro y punteros a los nodos hijos izquierdo y derecho, así como la altura del nodo para mantener el balance del árbol.
struct Nodo {
    libro: Libro,
    // Box<T> es un puntero inteligente que asigna memoria en el heap.
    // Se usa aquí para permitir que la estructura sea recursiva, ya que Box tiene un tamaño fijo.
    izquierdo: Option<Box<Nodo>>,
    // El uso de Option<Box<Nodo>> permite representar la ausencia de un hijo (None) o la presencia de un nodo (Some(Box<Nodo>)).
    derecho: Option<Box<Nodo>>,
    // La altura se mantiene para cada nodo para facilitar el cálculo del balance y las rotaciones necesarias para mantener el árbol AVL equilibrado.
    altura: i32,
}

// Implementación de métodos para la estructura Nodo.
impl Nodo {
    fn nuevo(libro: Libro) -> Self {
        Nodo {
            libro,
            izquierdo: None,
            derecho: None,
            altura: 1,
        }
    }
}
// Función auxiliar para obtener la altura de un nodo. Si el nodo es None, se considera que su altura es 0.
fn obtener_altura(nodo: &Option<Box<Nodo>>) -> i32 {
    // as_ref() convierte un &Option<T> en Option<&T>, permitiendo acceder al contenido
    // por referencia sin tomar la propiedad del valor original.
    nodo.as_ref().map_or(0, |n| n.altura)
}
// Función auxiliar para actualizar la altura de un nodo después de una inserción o eliminación. La altura se calcula como 1 más el máximo entre las alturas de los hijos izquierdo y derecho.
fn actualizar_altura(nodo: &mut Nodo) {
    nodo.altura = 1 + std::cmp::max(
        obtener_altura(&nodo.izquierdo),
        obtener_altura(&nodo.derecho),
    );
}
// Función para calcular el factor de balance de un nodo, que es la diferencia entre la altura del subárbol izquierdo y la altura del subárbol derecho. Un valor de balance de -1, 0 o 1 indica que el nodo está equilibrado, mientras que valores fuera de este rango indican que el nodo está desbalanceado y requiere rotación.
fn obtener_balance(nodo: &Nodo) -> i32 {
    obtener_altura(&nodo.izquierdo) - obtener_altura(&nodo.derecho)
}
// ROTACIÓN SIMPLE DERECHA (caso Left-Left)
//
//      y                x
//     / \              / \
//    x   T3    →     T1   y
//   / \                  / \
//  T1  T2              T2  T3
//
// Por qué .take():
// y es Box<Nodo> y lo poseemos. Si escribiéramos let mut x = y.izquierdo
// directamente, Rust daría error: no se puede mover un campo de un valor
// que sigue en uso. .take() reemplaza y.izquierdo con None y devuelve el
// valor anterior; así el campo queda en estado válido y el borrow checker
// acepta la operación.
// -------------------------------------------------------------------------------
fn rotar_derecha(mut y: Box<Nodo>) -> Box<Nodo> {
    // take() extrae el valor contenido en un Option, dejando un None en su lugar.
    // Esto permite mover la propiedad del nodo hijo fuera del padre.
    let mut x = y.izquierdo.take().expect("Hijo izquierdo ausente");
    y.izquierdo = x.derecho.take();
    actualizar_altura(&mut y);
    x.derecho = Some(y);
    actualizar_altura(&mut x);
    x
}

// ---------------------------------------------------------------
// ROTACIÓN SIMPLE IZQUIERDA (caso Right-Right)
//
//   x                  y
//  / \                / \
// T1   y      →      x   T3
//     / \           / \
//    T2  T3        T1  T2
// ------------------------------------------------------
fn rotar_izquierda(mut x: Box<Nodo>) -> Box<Nodo> {
    let mut y = x.derecho.take().expect("Hijo derecho ausente");
    x.derecho = y.izquierdo.take();
    actualizar_altura(&mut x);
    y.izquierdo = Some(x);
    actualizar_altura(&mut y);
    y
}

// Función de inserción que mantiene el balance del árbol AVL. Recibe un nodo opcional (puede ser None para el caso base) y un libro a insertar. Devuelve el nuevo nodo raíz del subárbol después de la inserción y las posibles rotaciones.
fn insertar(nodo_opt: Option<Box<Nodo>>, libro: Libro) -> Box<Nodo> {
    let mut nodo = match nodo_opt {
        None => return Box::new(Nodo::nuevo(libro)),
        Some(n) => n,
    };

    let isbn_nuevo = libro.isbn;

    if isbn_nuevo < nodo.libro.isbn {
        nodo.izquierdo = Some(insertar(nodo.izquierdo.take(), libro));
    } else if isbn_nuevo > nodo.libro.isbn {
        nodo.derecho = Some(insertar(nodo.derecho.take(), libro));
    } else {
        return nodo;
    }

    actualizar_altura(&mut nodo);
    let balance = obtener_balance(&nodo);

    if balance > 1 && isbn_nuevo < nodo.izquierdo.as_ref().unwrap().libro.isbn {
        return rotar_derecha(nodo);
    }
    if balance < -1 && isbn_nuevo > nodo.derecho.as_ref().unwrap().libro.isbn {
        return rotar_izquierda(nodo);
    }
    if balance > 1 && isbn_nuevo > nodo.izquierdo.as_ref().unwrap().libro.isbn {
        let hijo_izq = nodo.izquierdo.take().unwrap();
        nodo.izquierdo = Some(rotar_izquierda(hijo_izq));
        return rotar_derecha(nodo);
    }
    if balance < -1 && isbn_nuevo < nodo.derecho.as_ref().unwrap().libro.isbn {
        let hijo_der = nodo.derecho.take().unwrap();
        nodo.derecho = Some(rotar_derecha(hijo_der));
        return rotar_izquierda(nodo);
    }
    nodo
}

// Función de impresión del árbol en orden (in-order) para mostrar los libros ordenados por ISBN. Recibe una referencia al nodo raíz y un nivel para controlar la indentación visual.
fn imprimir(nodo: &Option<Box<Nodo>>, nivel: usize) {
    if let Some(n) = nodo {
        imprimir(&n.derecho, nivel + 1);
        println!(
            "{:indent$}[ISBN: {}] {}",
            "",
            n.libro.isbn,
            n.libro.titulo,
            indent = nivel * 4
        );
        imprimir(&n.izquierdo, nivel + 1);
    }
}

// ================================================================
// FASE 2: BÚSQUEDA EFICIENTE
// Firma: fn buscar(nodo: &Option<Box<Nodo>>, isbn: u32) -> Option<&Libro>
//
// Retorna una referencia al Libro (sin clonar ni copiar datos).
// Recorre el árbol igual que un BST binario: O(log n) garantizado
// porque el AVL mantiene el balance.
// ================================================================
fn buscar(nodo: &Option<Box<Nodo>>, isbn: u32) -> Option<&Libro> {
    // Comprobar si el nodo actual existe
    if let Some(n) = nodo {
        // Si el ISBN coincide, devolvemos una referencia al libro del nodo actual.
        // Usamos as_ref() para obtener una referencia sin tomar posesión.
        if isbn == n.libro.isbn {
            return Some(&n.libro);
        }

        // Si el ISBN buscado es menor, buscamos recursivamente en el subárbol izquierdo.
        if isbn < n.libro.isbn {
            return buscar(&n.izquierdo, isbn);
        }

        // Si el ISBN buscado es mayor, buscamos recursivamente en el subárbol derecho.
        return buscar(&n.derecho, isbn);
    }

    // Si llegamos a un nodo None, el libro no existe en el árbol.
    None
}

// ================================================================
// FASE 3: ELIMINACIÓN CON REBALANCEO
// Firma: fn eliminar(nodo_opt: Option<Box<Nodo>>, isbn: u32) -> Option<Box<Nodo>>
//
// Maneja 3 casos:
//   1. Nodo hoja (sin hijos)      → simplemente se elimina
//   2. Nodo con un solo hijo      → el hijo reemplaza al nodo
//   3. Nodo con dos hijos         → se reemplaza con el sucesor in-orden
//      (el nodo más pequeño del subárbol derecho) y luego se elimina
//      ese sucesor del subárbol derecho.
//
// Tras eliminar: actualiza altura y aplica rotaciones si es necesario.
// ================================================================

// Función auxiliar: extrae el nodo con ISBN mínimo de un subárbol.
// Retorna (libro_del_mínimo, nuevo_subárbol_sin_ese_nodo).
fn eliminar(nodo: Option<Box<Nodo>>, isbn: u32) -> Option<Box<Nodo>> {
    let mut nodo = match nodo {
        None => return None,
        Some(n) => n,
    };

    if isbn < nodo.libro.isbn {
        nodo.izquierdo = eliminar(nodo.izquierdo.take(), isbn);
    } else if isbn > nodo.libro.isbn {
        nodo.derecho = eliminar(nodo.derecho.take(), isbn);
    } else {
        if nodo.izquierdo.is_none() {
            return nodo.derecho;
        } else if nodo.derecho.is_none() {
            return nodo.izquierdo;
        }

        // 1. Recorremos hasta el mínimo del subárbol derecho (sucesor in-orden)
        //    dentro de un bloque acotado para que el borrow mutable termine
        //    antes de llamar a eliminar(). Sin este bloque, `sucesor` mantendría
        //    un &mut sobre `nodo.derecho` activo al mismo tiempo que .take() lo
        //    necesita, lo que viola las reglas del borrow-checker (E0499).
        let isbn_sucesor = {
            let mut sucesor = nodo.derecho.as_mut().unwrap();
            while let Some(ref mut izq) = sucesor.izquierdo {
                sucesor = izq;
            }
            // Copiamos el ISBN y el título antes de que el borrow termine.
            // Clone es necesario porque Libro no implementa Copy.
            let datos_sucesor = sucesor.libro.clone();
            nodo.libro = datos_sucesor; // reemplazamos el dato del nodo actual
            nodo.libro.isbn                // devolvemos el ISBN para la eliminación
        }; // ← aquí termina el borrow mutable sobre nodo.derecho

        // 2. Ahora podemos llamar a eliminar() con .take() sin conflicto.
        nodo.derecho = eliminar(nodo.derecho.take(), isbn_sucesor);
    }

    actualizar_altura(&mut nodo);
    let balance = obtener_balance(&nodo);

    if balance > 1 && obtener_balance(nodo.izquierdo.as_ref().unwrap()) >= 0 {
        return Some(rotar_derecha(nodo));
    }
    if balance > 1 && obtener_balance(nodo.izquierdo.as_ref().unwrap()) < 0 {
        let hijo_izq = nodo.izquierdo.take().unwrap();
        nodo.izquierdo = Some(rotar_izquierda(hijo_izq));
        return Some(rotar_derecha(nodo));
    }
    if balance < -1 && obtener_balance(nodo.derecho.as_ref().unwrap()) <= 0 {
        return Some(rotar_izquierda(nodo));
    }
    if balance < -1 && obtener_balance(nodo.derecho.as_ref().unwrap()) > 0 {
        let hijo_der = nodo.derecho.take().unwrap();
        nodo.derecho = Some(rotar_derecha(hijo_der));
        return Some(rotar_izquierda(nodo));
    }
    Some(nodo)
}
// ─── Estadísticas ────────────────────────────────────────────────────────────
// ================================================================
// FASE 4 — OPCIÓN B: ESTADÍSTICAS DEL ÁRBOL
// Retorna: (altura_total, total_nodos, libro_con_isbn_mas_alto)
// ================================================================
struct EstadisticasArbol<'a> {
    altura_total: usize,
    total_nodos: usize,
    libro_isbn_mayor: Option<&'a Libro>,
}


fn obtener_estadisticas<'a>(nodo: &'a Option<Box<Nodo>>) -> EstadisticasArbol<'a> {
    match nodo {
        // Caso base: nodo vacío → valores neutros.
        None => EstadisticasArbol {
            altura_total: 0,
            total_nodos: 0,
            libro_isbn_mayor: None,
        },

        Some(n) => {
            // Calculamos estadísticas de cada subárbol de forma independiente.
            let izq = obtener_estadisticas(&n.izquierdo);
            let der = obtener_estadisticas(&n.derecho);

            // Altura: el campo `altura` ya está mantenido por insertar/eliminar,
            // por lo que basta leerlo en la raíz. Para el recorrido recursivo
            // usamos max() entre subárboles y sumamos 1 por el nodo actual.
            let altura_total = 1 + std::cmp::max(izq.altura_total, der.altura_total);

            // Nodos: sumamos ambos subárboles más el nodo actual.
            let total_nodos = 1 + izq.total_nodos + der.total_nodos;

            // ISBN mayor: en un BST el máximo siempre está en el extremo derecho.
            // Si el subárbol derecho tiene candidato lo comparamos; si no, el
            // nodo actual es el máximo de este subárbol.
            // Option::map_or_else evita unwrap() innecesario.
            let libro_isbn_mayor = match der.libro_isbn_mayor {
                // El subárbol derecho tiene nodos: su máximo ya es mayor que el
                // nodo actual (propiedad BST), lo devolvemos directamente.
                Some(libro_der) => Some(libro_der),
                // El subárbol derecho está vacío: el nodo actual es el máximo
                // de este subárbol.
                None => Some(&n.libro),
            };

            EstadisticasArbol {
                altura_total,
                total_nodos,
                libro_isbn_mayor,
            }
        }
    }
}

// En el main, demostramos la funcionalidad del sistema de inventario de librería utilizando un árbol AVL. Insertamos varios libros, imprimimos el árbol, realizamos búsquedas y eliminaciones, y finalmente obtenemos estadísticas del árbol.
fn main() {
    println!("╔══════════════════════════════════════════════════╗");
    println!("║   Sistema de Inventario de Librería (AVL) - SA   ║");
    println!("╚══════════════════════════════════════════════════╝\n");
    let mut raiz: Option<Box<Nodo>> = None;
    let datos = vec![
        (10, "El Quijote"),
        (20, "1984"),
        (30, "Hamlet"),
        (5, "Fahrenheit 451"),
        (2, "La Odisea"),
        (25, "El Principito"),
    ];

    println!("--- Sistema de Inventario de Librería (AVL) ---");
    for (isbn, titulo) in datos {
        let libro = Libro {
            isbn,
            titulo: titulo.to_string(),
        };
        raiz = Some(insertar(raiz.take(), libro));
    }

    imprimir(&raiz, 0);

    // --- ESPACIO PARA TUS PRUEBAS ---
    println!("\n─────────────────────────────────────────────────");
    println!(" FASE 2 — Pruebas de búsqueda");
    println!("─────────────────────────────────────────────────");
    // Buscamos un libro existente
     let isbn_buscar = 20;
    match buscar(&raiz, isbn_buscar) {
        Some(libro) => println!("Libro encontrado: [ISBN: {}] {}", libro.isbn, libro.titulo),
        None => println!("Libro con ISBN {} no encontrado.", isbn_buscar),
    }

    // Buscamos un libro inexistente
    let isbn_inexistente = 99;
    match buscar(&raiz, isbn_inexistente) {
        Some(libro) => println!("Libro encontrado: [ISBN: {}] {}", libro.isbn, libro.titulo),
        None => println!("Libro con ISBN {} no encontrado.", isbn_inexistente),
    }

    // --- PRUEBAS DE ELIMINACIÓN ---
    println!("\n─────────────────────────────────────────────────");
    println!("  FASE 3 — Pruebas de eliminación");
    println!("─────────────────────────────────────────────────");
    println!("\n--- Eliminando ISBN 20 (nodo con sucesor 25) ---");
    raiz = eliminar(raiz, 20);
    imprimir(&raiz, 0);
    println!("\n--- Eliminando ISBN 2 (nodo hoja) ---");
    raiz = eliminar(raiz, 2);
    imprimir(&raiz, 0);

    println!("\n--- Eliminando ISBN 10 (nodo con un hijo) ---");
    raiz = eliminar(raiz, 10);
    imprimir(&raiz, 0);

    // ── Estadísticas ──────────────────────────────────────────────────────────
    println!("\n─────────────────────────────────────────────────");
    println!(" FASE 4 — Estadísticas del árbol");
    println!("─────────────────────────────────────────────────");

    println!("\n=== Estadísticas (árbol completo inicial) ===");
    let mut raiz_stats: Option<Box<Nodo>> = None;
    for (isbn, titulo) in [(10, "El Quijote"), (20, "1984"), (30, "Hamlet"),
                            (5, "Fahrenheit 451"), (2, "La Odisea"), (25, "El Principito")] {
        raiz_stats = Some(insertar(raiz_stats.take(), Libro { isbn, titulo: titulo.to_string() }));
    }

    let stats = obtener_estadisticas(&raiz_stats);
    println!("  Altura total del árbol : {}", stats.altura_total);
    println!("  Total de nodos         : {}", stats.total_nodos);
    match stats.libro_isbn_mayor {
        Some(libro) => println!(
            "  Libro con ISBN mayor   : [ISBN: {}] \"{}\"",
            libro.isbn, libro.titulo
        ),
        None => println!("  El árbol está vacío."),
    }

    // Verificamos que las estadísticas se mantienen correctas tras una eliminación.
    println!("\n=== Estadísticas tras eliminar ISBN 30 (Hamlet) ===");
    raiz_stats = eliminar(raiz_stats, 30);
    let stats2 = obtener_estadisticas(&raiz_stats);
    println!("  Altura total del árbol : {}", stats2.altura_total);
    println!("  Total de nodos         : {}", stats2.total_nodos);
    match stats2.libro_isbn_mayor {
        Some(libro) => println!(
            "  Libro con ISBN mayor   : [ISBN: {}] \"{}\"",
            libro.isbn, libro.titulo
        ),
        None => println!("  El árbol está vacío."),
    }

    // Árbol vacío → todos los valores en su estado neutro.
    println!("\n=== Estadísticas de árbol vacío ===");
    let stats3 = obtener_estadisticas(&None);
    println!("  Altura total del árbol : {}", stats3.altura_total);
    println!("  Total de nodos         : {}", stats3.total_nodos);
    match stats3.libro_isbn_mayor {
        Some(libro) => println!("  Libro con ISBN mayor   : [ISBN: {}] \"{}\"", libro.isbn, libro.titulo),
        None => println!("  Libro con ISBN mayor   : (árbol vacío)"),
    }
    println!("\n╔══════════════════════════════════════════════════╗");
    println!("║           Todas las pruebas completadas        ║");
    println!("╚══════════════════════════════════════════════════╝");
}

// En Rust, usar .take() (por ejemplo, en un Option<Node>) es necesario porque el sistema de propiedad (ownership) prohíbe dejar una variable temporalmente vacía o sin inicializar al mover su contenido.
// Una asignación directa intentaría extraer el valor dejando al nodo padre en un estado inválido, lo cual activa el verificador de préstamos (borrow checker).
// En su lugar, .take() extrae el valor de forma segura reemplazándolo instantáneamente por un None en el nodo original. Esto permite reestructurar los punteros durante la rotación sin violar las reglas de seguridad de memoria de Rust.