#[derive(Debug)]
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

// Method bajo contexto de estructura como implementación
/*
Para definir la función en el contexto de Rectangle, iniciamos un impl bloque (de implementación) para Rectangle. All dentro de este impl bloque se asociará con el Rectangletipo. Luego, movemos la area función dentro de las impl llaves y cambiamos el primer (y en este caso, el único) parámetro para que esté self en la firma y en all el cuerpo. En main, donde llamamos a la area función y la pasamos rect1como argumento, podemos usar la sintaxis de Method para llamar al area Method en nuestra Rectangle instancia. La sintaxis de Method va después de una instancia: añadimos un punto seguido del nombre del Method, los paréntesis y los argumentos.
En la firma de area, usamos &self en lugar de rectangle: &Rectangle. The &self es en realidad la abreviatura de self: &Self. Dentro de un impl bloque, el tipo Self es un alias del tipo al que impl pertenece el bloque. Los métodos deben tener un parámetro llamado" self of type" Self como primer parámetro, por lo que Rust permite abreviarlo con solo el nombre self en el primer parámetro. Tenga en cuenta que aún necesitamos usar "the" &antes de la self abreviatura para indicar que este Method toma prestada la Self instancia, tal como hicimos en rectangle: &Rectangle. Los métodos pueden tomar posesión de self, tomar prestada self inmutablemente, como hicimos aquí, o tomar prestada self mutablemente, al igual que cualquier otro parámetro.
Elegimos &self esta opción por la misma razón que &Rectangle en la versión de función: no queremos tomar posesión de la instancia, sino leer los datos de la estructura, no escribir en ella. Si quisiéramos cambiar la instancia en la que invocamos el Method como parte de su función, usaríamos ``` &mut self como primer parámetro. Es poco común que un Method tome posesión de la instancia usando ``` self como primer parámetro; esta técnica se suele usar cuando el Method se transforma self en otra cosa y se desea evitar que quien lo llama use la instancia original después de la transformación.
La razón principal para usar métodos en lugar de funciones, además de proporcionar la sintaxis de Method y evitar la repetición del tipo de self en la firma de cada Method, es la organización. Hemos agrupado todas las funciones que podemos realizar con una instancia de un tipo en un solo impl bloque, en lugar de obligar a los futuros usuarios de nuestro código a buscar las capacidades de "of" Rectangle en diversas partes de la biblioteca que ofrecemos.
 */
impl Rectangle {
    pub fn area(&self) -> u32 {
        self.width * self.height
    }
}