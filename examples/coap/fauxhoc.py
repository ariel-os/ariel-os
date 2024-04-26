"""
fake-it-until-you-make-it wrapper for sending an EDHOC+OSCORE request

Its first stage did roughly

~~~
$ ./aiocoap-client 'coap://10.42.0.61/.well-known/edhoc' -m POST --content-format application/cbor-seq --payload "[true, 3, 2, h'0000000000000000000000000000000000000000000000000000000000000000', 0]"
~~~

and the later guessed the C_R and sent a garbled OSCORE request that triggered
the EDHOC CoAP pathways.

Now it does EDHOC using lakers to a point where the name doesn't fit any more…
"""

import asyncio
import cbor2
from aiocoap import *
from aiocoap import numbers, oscore

import lakers

# Someone told us that these are the credentials of devices that are our legitimate peers
eligible_responders_ccs = {
    bytes.fromhex(
        "A2026008A101A5010202410A2001215820BBC34960526EA4D32E940CAD2A234148DDC21791A12AFBCBAC93622046DD44F02258204519E257236B2A0CE2023F0931F1F386CA7AFDA64FCDE0108C224C51EABF6072"
    )
}
eligible_responders = {}  # mapping ID_CRED_R to CRED_R
# when ID_CRED_R is the KID. 8/1/2 is cnf/COSE_Key/kid, IIUC those should be present in suitable CCSs
eligible_responders |= {
    parsed[8][1][2]: ccs
    for (parsed, ccs) in ((cbor2.loads(ccs), ccs) for ccs in eligible_responders_ccs)
}
# when ID_CRED_R is CRED_R
eligible_responders |= {ccs: ccs for ccs in eligible_responders_ccs}

CRED_I = bytes.fromhex(
    "A2027734322D35302D33312D46462D45462D33372D33322D333908A101A5010202412B2001215820AC75E9ECE3E50BFC8ED60399889522405C47BF16DF96660A41298CB4307F7EB62258206E5DE611388A4B8A8211334AC7D37ECB52A387D257E6DB3C2A93DF21FF3AFFC8"
)
I = bytes.fromhex("fb13adeb6518cee5f88417660841142e830a81fe334380a953406a1305e8706b")


class EdhocSecurityContext(
    oscore.CanProtect, oscore.CanUnprotect, oscore.SecurityContextUtils
):
    def __init__(self, initiator, c_ours, c_theirs):
        # initiator could also be responder, and only this line would need to change
        # FIXME Only ByReference implemented in edhoc.rs so far
        self.message_3, _i_prk_out = initiator.prepare_message_3(
            lakers.CredentialTransfer.ByReference, None
        )

        if initiator.selected_cipher_suite() == 2:
            self.alg_aead = oscore.algorithms["AES-CCM-16-64-128"]
            self.hashfun = oscore.hashfunctions["sha256"]
        else:
            raise RuntimeError("Unknown suite")

        # we check critical EADs, no out-of-band agreement, so 8 it is
        oscore_salt_length = 8
        # I figure that one would be ageed out-of-band as well
        self.id_context = None
        self.recipient_replay_window = oscore.ReplayWindow(32, lambda: None)

        master_secret = initiator.edhoc_exporter(0, [], self.alg_aead.key_bytes)
        master_salt = initiator.edhoc_exporter(1, [], oscore_salt_length)
        print(f"Derived {master_secret=} {master_salt=}")

        self.sender_id = cbor2.dumps(c_theirs)
        self.recipient_id = cbor2.dumps(c_ours)
        if self.sender_id == self.recipient_id:
            raise ValueError("Bad IDs: identical ones were picked")

        self.derive_keys(master_salt, master_secret)

        self.sender_sequence_number = 0
        self.recipient_replay_window.initialize_empty()

    def post_seqnoincrease(self):
        pass

    def protect(self, message, request_id=None, *, kid_context=True):
        outer_message, request_id = super().protect(
            message, request_id=request_id, kid_context=kid_context
        )
        if self.message_3 is not None:
            outer_message.opt.edhoc = True
            outer_message.payload = self.message_3 + outer_message.payload
            self.message_3 = None
        return outer_message, request_id


async def main():
    ctx = await Context.create_client_context()

    priv, pub = lakers.p256_generate_key_pair()

    c_i = 0x08
    initiator = lakers.EdhocInitiator()
    message_1 = initiator.prepare_message_1(c_i=c_i)

    msg1 = Message(
        code=POST,
        uri="coap://10.42.0.61/.well-known/edhoc",
        payload=cbor2.dumps(True) + message_1,
        # payload=b"".join(cbor2.dumps(x) for x in [True, 3, 2, b'\0' * 32, 0]),
    )
    msg2 = await ctx.request(msg1).response_raising

    (c_r, id_cred_r, ead_2) = initiator.parse_message_2(msg2.payload)
    # https://github.com/openwsn-berkeley/lakers/issues/256 -- conveniently,
    # after it is fixed, the line becomes a no-op, so this can stay in until a
    # Lakers release to PyPI happened.
    id_cred_r = bytes(id_cred_r)

    print(f"Received MSG2 with {c_r=} {id_cred_r=} {ead_2=}")

    cred_r = eligible_responders[id_cred_r]
    initiator.verify_message_2(
        I, CRED_I, cred_r
    )  # odd that we provide that here rather than in the next function

    oscore_context = EdhocSecurityContext(initiator, c_i, c_r)

    ctx.client_credentials["coap://10.42.0.61/*"] = oscore_context

    msg3 = Message(
        code=GET,
        uri="coap://10.42.0.61/.well-known/core",
    )

    print((await ctx.request(msg3).response_raising).payload)

    normalrequest = Message(
        code=GET,
        uri="coap://10.42.0.61/poem",
    )
    print((await ctx.request(normalrequest).response).payload)

    await ctx.shutdown()


if __name__ == "__main__":
    asyncio.run(main())
