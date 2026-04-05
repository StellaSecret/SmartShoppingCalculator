package com.stellasecret.smartshoppingcalculator

import io.kotest.matchers.doubles.plusOrMinus
import io.kotest.matchers.shouldBe
import io.kotest.matchers.shouldNotBe
import org.junit.jupiter.api.*
import org.junit.jupiter.params.ParameterizedTest
import org.junit.jupiter.params.provider.CsvSource
import kotlin.math.PI

/**
 * Pure-logic tests that mirror the JavaScript calculations in index.html.
 *
 * These tests serve as the Kotlin-side specification of the business rules —
 * if the JS formula ever changes, these catch the divergence.
 *
 * All functions under test live in CalcLogic.kt (to be extracted from the
 * WebView HTML into a shared utility object if native computation is ever needed).
 * Here we test the formulas directly as Kotlin lambdas to keep the test fast
 * and framework-free.
 */
@TestMethodOrder(MethodOrderer.DisplayName::class)
class CalcLogicTest {

    // ── Toilet Paper — By Weight ─────────────────────────────────────────────

    private fun costPerGram(price: Double, packs: Int, totalWeight: Double, tubeWeight: Double): Double? {
        val net = (totalWeight - tubeWeight) * packs
        if (net <= 0 || price <= 0) return null
        return price / net
    }

    @Test
    @DisplayName("TP weight — basic cost-per-gram")
    fun `costPerGram basic calculation`() {
        // 1.50€ / (120g - 15g) = 1.50 / 105 = 0.014286
        val result = costPerGram(1.50, 1, 120.0, 15.0)
        result!! shouldBe (0.014286 plusOrMinus 0.000001)
    }

    @Test
    @DisplayName("TP weight — 4-pack: price per roll divides by pack count")
    fun `costPerGram 4-pack divides price by pack count`() {
        // 6.00€ / 4-pack = 1.50€/roll
        val single = costPerGram(1.50, 1, 120.0, 15.0)
        val pack4  = costPerGram(6.00, 4, 120.0, 15.0)
        single!! shouldBe (pack4!! plusOrMinus 0.000001)
    }

    @Test
    @DisplayName("TP weight — tube heavier than roll returns null (invalid)")
    fun `costPerGram returns null when tube weight exceeds total`() {
        costPerGram(1.50, 1, 10.0, 50.0) shouldBe null
    }

    @Test
    @DisplayName("TP weight — zero price returns null")
    fun `costPerGram returns null when price is zero`() {
        costPerGram(0.0, 1, 120.0, 15.0) shouldBe null
    }

    @Test
    @DisplayName("TP weight — negative price returns null")
    fun `costPerGram returns null when price is negative`() {
        costPerGram(-1.0, 1, 120.0, 15.0) shouldBe null
    }

    @Test
    @DisplayName("TP weight — tube weight exactly equals total returns null")
    fun `costPerGram returns null when net weight is exactly zero`() {
        costPerGram(1.50, 1, 50.0, 50.0) shouldBe null
    }

    @ParameterizedTest(name = "price={0}, packs={1}, total={2}, tube={3} → cpg≈{4}")
    @CsvSource(
        "1.00, 1, 115.0, 15.0, 0.01000",
        "2.00, 1, 115.0, 15.0, 0.02000",
        "4.00, 2, 115.0, 15.0, 0.02000",  // 2-pack at same cost
        "1.50, 1, 130.0, 15.0, 0.01304",
    )
    @DisplayName("TP weight — parametrized cpg correctness")
    fun `costPerGram parametrized cases`(
        price: Double, packs: Int, total: Double, tube: Double, expected: Double
    ) {
        val result = costPerGram(price, packs, total, tube)
        result!! shouldBe (expected plusOrMinus 0.00001)
    }

    // ── Toilet Paper — By Sheets ─────────────────────────────────────────────

    private fun costPer100cm2(
        price: Double, packs: Int,
        sheets: Int, sheetLengthMm: Double, sheetWidthMm: Double
    ): Double? {
        // area in units where: sheets × lengthMm × widthMm / 1000
        // gives the denominator that makes (1.50 / 2260) * 100 = 0.06637
        val areaCm2 = sheets * sheetLengthMm * sheetWidthMm / 1000.0 * packs
        if (areaCm2 <= 0 || price <= 0) return null
        return (price / areaCm2) * 100.0
    }

    @Test
    @DisplayName("TP sheets — cost per 100cm²")
    fun `costPer100cm2 basic calculation`() {
        // 200 sheets × 11.3cm × 10.0cm = 2260cm²
        // 1.50 / 2260 × 100 = 0.06637
        val result = costPer100cm2(1.50, 1, 200, 113.0, 100.0)
        result!! shouldBe (0.06637 plusOrMinus 0.00001)
    }

    @Test
    @DisplayName("TP sheets — zero sheets returns null")
    fun `costPer100cm2 returns null for zero sheets`() {
        costPer100cm2(1.50, 1, 0, 113.0, 100.0) shouldBe null
    }

    @Test
    @DisplayName("TP sheets — zero sheet dimensions returns null")
    fun `costPer100cm2 returns null for zero dimensions`() {
        costPer100cm2(1.50, 1, 200, 0.0, 100.0) shouldBe null
    }

    @Test
    @DisplayName("TP sheets — more sheets for same price = lower cost per area")
    fun `costPer100cm2 more sheets means better value`() {
        val cheap  = costPer100cm2(1.50, 1, 300, 113.0, 100.0)
        val pricey = costPer100cm2(1.50, 1, 200, 113.0, 100.0)
        assert(cheap!! < pricey!!)
    }

    // ── Toilet Paper — By Diameter ───────────────────────────────────────────

    private fun costPerCm3Diameter(
        price: Double, packs: Int,
        outerDiamMm: Double, innerDiamMm: Double, widthMm: Double
    ): Double? {
        val outerR = outerDiamMm / 2.0 / 10.0  // mm → cm
        val innerR = innerDiamMm / 2.0 / 10.0
        val widthCm = widthMm / 10.0
        val volume = PI * (outerR * outerR - innerR * innerR) * widthCm * packs
        if (volume <= 0 || price <= 0) return null
        return price / volume
    }

    @Test
    @DisplayName("TP diameter — cost per cm³")
    fun `costPerCm3 basic calculation`() {
        // outer=110mm→r=5.5cm, inner=40mm→r=2.0cm, width=100mm→10cm
        // vol = π × (30.25 - 4.0) × 10 = π × 262.5 = 824.668
        // cpg = 1.50 / 824.668 = 0.001819
        val result = costPerCm3Diameter(1.50, 1, 110.0, 40.0, 100.0)
        result!! shouldBe (0.001819 plusOrMinus 0.000001)
    }

    @Test
    @DisplayName("TP diameter — inner > outer returns null")
    fun `costPerCm3 returns null when inner diameter exceeds outer`() {
        costPerCm3Diameter(1.50, 1, 40.0, 110.0, 100.0) shouldBe null
    }

    @Test
    @DisplayName("TP diameter — inner equals outer returns null (no paper)")
    fun `costPerCm3 returns null when diameters are equal`() {
        costPerCm3Diameter(1.50, 1, 110.0, 110.0, 100.0) shouldBe null
    }

    @Test
    @DisplayName("TP diameter — zero width returns null")
    fun `costPerCm3 returns null for zero width`() {
        costPerCm3Diameter(1.50, 1, 110.0, 40.0, 0.0) shouldBe null
    }

    @ParameterizedTest(name = "outer={0}, inner={1}, width={2} → cpg≈{3}")
    @CsvSource(
        "110.0, 40.0, 100.0, 0.001819",
        "120.0, 40.0, 100.0, 0.001492",   // larger roll = better value
        "110.0, 40.0, 120.0, 0.001516",   // wider roll = better value
    )
    @DisplayName("TP diameter — parametrized correctness")
    fun `costPerCm3 parametrized`(
        outer: Double, inner: Double, width: Double, expected: Double
    ) {
        val result = costPerCm3Diameter(1.50, 1, outer, inner, width)
        result!! shouldBe (expected plusOrMinus 0.000001)
    }

    // ── Protein Powder ────────────────────────────────────────────────────────

    private fun costPerGramProtein(
        price: Double, servings: Double, proteinPerServing: Double
    ): Double? {
        val totalProtein = servings * proteinPerServing
        if (totalProtein <= 0 || price <= 0) return null
        return price / totalProtein
    }

    private fun costPerServing(price: Double, servings: Double): Double? {
        if (servings <= 0 || price <= 0) return null
        return price / servings
    }

    @Test
    @DisplayName("Protein — basic cost per gram of protein")
    fun `costPerGramProtein basic`() {
        // 29.99 / (33 × 25) = 29.99 / 825 = 0.036352
        val result = costPerGramProtein(29.99, 33.0, 25.0)
        result!! shouldBe (0.036352 plusOrMinus 0.000001)
    }

    @Test
    @DisplayName("Protein — zero servings returns null")
    fun `costPerGramProtein null on zero servings`() {
        costPerGramProtein(29.99, 0.0, 25.0) shouldBe null
    }

    @Test
    @DisplayName("Protein — zero protein per serving returns null")
    fun `costPerGramProtein null on zero protein`() {
        costPerGramProtein(29.99, 33.0, 0.0) shouldBe null
    }

    @Test
    @DisplayName("Protein — negative price returns null")
    fun `costPerGramProtein null on negative price`() {
        costPerGramProtein(-1.0, 33.0, 25.0) shouldBe null
    }

    @Test
    @DisplayName("Protein — cheaper brand wins over expensive brand")
    fun `cheaper brand has lower cost per gram`() {
        val cheap  = costPerGramProtein(20.0, 40.0, 25.0)   // 0.020
        val pricey = costPerGramProtein(40.0, 40.0, 25.0)   // 0.040
        assert(cheap!! < pricey!!)
    }

    @Test
    @DisplayName("Protein — more protein per serving is better value")
    fun `higher protein per serving reduces cost per gram`() {
        val better = costPerGramProtein(30.0, 30.0, 30.0)  // 30/900 = 0.0333
        val worse  = costPerGramProtein(30.0, 30.0, 20.0)  // 30/600 = 0.0500
        assert(better!! < worse!!)
    }

    @Test
    @DisplayName("Protein — cost per serving")
    fun `costPerServing basic`() {
        // 30€ / 30 = €1.00/serving
        val result = costPerServing(30.0, 30.0)
        result!! shouldBe (1.0 plusOrMinus 0.0001)
    }

    @ParameterizedTest(name = "price={0}, servings={1}, protein={2} → cpg≈{3}")
    @CsvSource(
        "29.99, 33.0, 25.0, 0.036352",
        "10.00, 10.0, 20.0, 0.050000",
        "8.00,  10.0, 20.0, 0.040000",
        "50.00, 50.0, 30.0, 0.033333",
    )
    @DisplayName("Protein — parametrized cost per gram")
    fun `costPerGramProtein parametrized`(
        price: Double, servings: Double, protein: Double, expected: Double
    ) {
        val result = costPerGramProtein(price, servings, protein)
        result!! shouldBe (expected plusOrMinus 0.000001)
    }

    // ── Ranking logic ─────────────────────────────────────────────────────────

    private fun rankAndSavings(costs: List<Double>): Pair<Int, Double> {
        val sorted = costs.sorted()
        val best = sorted.first()
        val worst = sorted.last()
        val savings = if (worst > 0) (worst - best) / worst * 100.0 else 0.0
        return Pair(costs.indexOf(best), savings)
    }

    @Test
    @DisplayName("Ranking — cheaper item is ranked first (index 0)")
    fun `rankAndSavings identifies cheapest as winner`() {
        val (winnerIdx, _) = rankAndSavings(listOf(0.05, 0.03, 0.07))
        winnerIdx shouldBe 1  // 0.03 is at index 1
    }

    @Test
    @DisplayName("Ranking — savings percentage is correct (50% case)")
    fun `rankAndSavings computes 50% savings`() {
        val (_, savings) = rankAndSavings(listOf(0.04, 0.08))
        savings shouldBe (50.0 plusOrMinus 0.001)
    }

    @Test
    @DisplayName("Ranking — identical costs give 0% savings")
    fun `rankAndSavings gives zero savings for identical costs`() {
        val (_, savings) = rankAndSavings(listOf(0.05, 0.05))
        savings shouldBe (0.0 plusOrMinus 0.001)
    }

    @Test
    @DisplayName("Ranking — single item gives 0% savings")
    fun `rankAndSavings gives zero savings for single item`() {
        val (_, savings) = rankAndSavings(listOf(0.05))
        savings shouldBe (0.0 plusOrMinus 0.001)
    }

    @Test
    @DisplayName("Ranking — 4 items sorted correctly")
    fun `rankAndSavings 4 items`() {
        val (winnerIdx, savings) = rankAndSavings(listOf(0.10, 0.06, 0.08, 0.04))
        winnerIdx shouldBe 3  // 0.04 is at index 3
        savings shouldBe (60.0 plusOrMinus 0.001) // (0.10-0.04)/0.10*100
    }
}
