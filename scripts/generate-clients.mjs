#!/usr/bin/env zx
import "zx/globals";
import * as k from "@metaplex-foundation/kinobi";
import { getAllProgramIdls } from "./utils.mjs";

// Instanciate Kinobi.
const kinobi = k.createFromIdls(getAllProgramIdls());

// Update programs.
kinobi.update(
  k.updateProgramsVisitor({
    libreplexMonoswap: { name: "monoswap" }
  })
);

// Update instructions.
kinobi.update(
  k.updateInstructionsVisitor({
    createSwap: {
      byteDeltas: [k.instructionByteDeltaNode(k.accountLinkNode("swapMarker"))]
    }
  })
);

// Set account discriminators.
// const key = (name) => ({ field: "key", value: k.enumValueNode("Key", name) });
// kinobi.update(
//   k.setAccountDiscriminatorFromFieldVisitor({
//     counter: key("counter")
//   })
// );

// Render JavaScript Clients.a

// Render JavaScript (legacy).
const jsClientV1 = path.join(__dirname, "..", "clients", "js-v1");
kinobi.accept(
  k.renderJavaScriptVisitor(path.join(jsClientV1, "src", "generated"), {
    prettier: require(path.join(jsClientV1, ".prettierrc.json"))
  })
);

// Render JavaScript with new web3.js.
const jsClientV2 = path.join(__dirname, "..", "clients", "js-v2");
kinobi.accept(
  k.renderJavaScriptExperimentalVisitor(
    path.join(jsClientV2, "src", "generated"),
    { prettier: require(path.join(jsClientV2, ".prettierrc.json")) }
  )
);

// Render Rust.
const rustClient = path.join(__dirname, "..", "clients", "rust");
kinobi.accept(
  k.renderRustVisitor(path.join(rustClient, "src", "generated"), {
    formatCode: true,
    crateFolder: rustClient
  })
);
