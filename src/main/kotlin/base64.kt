/// Converting bytes to and from base64
import java.util.Base64

fun String.fromHexBytes(): ByteArray {
    check(length % 2 == 0 ) { "Malformed byte string"}
    return chunked(2).map {
        val int = it.toInt(16)
        int.toByte() }.toByteArray()
}
fun ByteArray.toBase64(): ByteArray =
    String(Base64.getEncoder().encode(this)).toByteArray()