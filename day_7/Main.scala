import scala.io.Source

object Main extends App {
    val bags = Util.parseFile("./input", "\r\n", line => line).map(Bag.from(_))

    val targetAttribute = "shiny"
    val targetColor = "gold"

    def find(bags: List[Bag], containing: List[Bag]): List[Bag] = {
        val results = bags
            .filter(b => b.containsAny(containing))
            .filterNot(b => containing.contains(b.name))

        if (results.length == 0) {
            List()
        } else {
            results ::: find(bags, results)
        }
    }

    def numberOfBagsContained(bag: Bag, bagList: List[Bag]): Int = {
        val rootBag = bagList.filter(_.name == bag.name).head

        println(rootBag)
        
        if (rootBag.contains.isEmpty) {
            0
        } else {
            rootBag.contains.foldLeft(0)((count, value) => count + (value._1) + (value._1 * numberOfBagsContained(value._2, bagList)))
        }
    }

    println(find(bags, List(Bag(targetAttribute, targetColor, List()))).map(b => b.name).distinct.length)

    val rootBag = bags.filter(b => b.attribute == targetAttribute && b.color == targetColor).head

    println(numberOfBagsContained(rootBag, bags))
}

object Bag {
    def from(rule: String): Bag = {
        val splitOnContain = rule.split("contain")

        val rootBag = splitOnContain(0)
            .replace("bag", "")
            .replace("bags", "")
            .trim()
            .split(" ")

        val containedBags = splitOnContain(1)
            .replace(".", "")
            .split(", ")
            .filter(_ != " no other bags")
            .map(value => {
                val bagData = value.trim().replace("bag", "").replace("bags", "").split(" ")
                (bagData(0).toInt, Bag(bagData(1), bagData(2), List()))
            })

        Bag(rootBag(0), rootBag(1), containedBags.toList)
    }
}

case class Bag(attribute: String, color: String, contains: List[(Int, Bag)]) {
    def containsAny(bags: List[Bag]): Boolean = contains.filter(c => bags.map(_.name).contains(c._2.name)).length > 0

    def name = attribute + "-" + color
}

object Util {
    def parseFile[A](fileName: String, split: String, func: String => A): List[A] = {
        Source.fromFile(fileName)
            .mkString
            .split(split)
            .map(func(_))
            .toList
    }
}
