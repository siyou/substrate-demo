import React, { useEffect, useMemo, useState } from "react";
import { Form, Grid } from "semantic-ui-react";

import { TxButton } from "./substrate-lib/components";
import { useSubstrateState } from "./substrate-lib";
import KittyCards from "./KittyCards";

export default function Main(props) {
  const [status, setStatus] = useState("");
  const [kitties, setKitties] = useState([]);

  const { api, currentAccount, keyring } = useSubstrateState();
  const accountIds = useMemo(
    () => keyring.getPairs().map((v) => v.address),
    [keyring]
  );

  useEffect(() => {
    const { allKitties, kitties } = api.query.kittiesModule;
    accountIds.every((address) =>
      allKitties(address)
        .then((ids) => {
          return Promise.allSettled(
            ids.map((id) => kitties(id).then((res) => [res, id]))
          );
        })
        .then((kitties) => {
          setKitties(
            kitties
              .filter((v) => v.status === "fulfilled")
              .map((v) => {
                const [result, id] = v.value;
                return {
                  owner: address,
                  id: id,
                  dna: result.value,
                };
              })
          );
        })
    );
  }, [accountIds, api.query.kittiesModule, status]);

  return (
    <Grid.Column width={16}>
      <h1>小毛孩</h1>
      <KittyCards
        kitties={kitties}
        accountPair={currentAccount}
        setStatus={setStatus}
      ></KittyCards>
      <Form style={{ margin: "1em 0" }}>
        <Form.Field style={{ textAlign: "center" }}>
          <TxButton
            label={`创建小毛孩(共${kitties.length}个)`}
            type="SIGNED-TX"
            setStatus={setStatus}
            attrs={{
              palletRpc: "kittiesModule",
              callable: "create",
              inputParams: [],
              paramFields: [],
            }}
          />
        </Form.Field>
      </Form>
      <div style={{ overflowWrap: "break-word" }}>{status}</div>
    </Grid.Column>
  );
}
