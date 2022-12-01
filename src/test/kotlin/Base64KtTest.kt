import org.junit.jupiter.api.Test

import org.junit.jupiter.api.Assertions.*

class Base64KtTest {

    @Test
    fun toBase64() {
        val bytes = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d"
        val decoded = bytes.fromHexBytes()
        val result = decoded.toBase64()
        val expected = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t".toByteArray()
        assertArrayEquals(result, expected)
    }
}