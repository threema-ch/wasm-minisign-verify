import {setupLogging, PublicKey, Signature} from 'wasm-minisign-verify';

document.getElementById("result").innerText = "Running…";

setupLogging("trace");

const pubkey = PublicKey.decode(
    "untrusted comment: minisign public key 60DF2F3B621B4533\n" +
    "RWQzRRtiOy/fYNCli5tW96CO6R+FnO92LceeIoWlCLj+BTVe+6q8T69M"
);

const signedData = new TextEncoder().encode("test\n");
const signature = Signature.decode(
    "untrusted comment: signature from minisign secret key\n" +
    "RWQzRRtiOy/fYEU/vGHUEfBg+lSmrdpViX3l9fX1Ps6FMBrBcsMw9uxsLPFr9pAMdKy1NVEX3MsHsuCKlSVNYc4C5/pCnU/Kugk=\n" +
    "trusted comment: timestamp:1634045550	file:test.txt\n" +
    "zEHzYWS0L/lFlN3hfMdAJA0MsVfazBXbwSw9XihxQ0msFQPlC30F6Ajvxi67KEFNd1GUhdi3DcslssTW8MUECQ=="
);

try {
    pubkey.verify(signedData, signature);
    document.getElementById("result").innerText = "Signature verification succeeded!";
} catch(e) {
    console.error(e);
    document.getElementById("result").innerText = "Signature verification failed: " + e;
}
