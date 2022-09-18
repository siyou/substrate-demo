import React, { useState } from "react";
import {
  Button,
  Card,
  Grid,
  Message,
  Modal,
  Form,
  Label,
  Dropdown
} from "semantic-ui-react";

import KittyAvatar from "./KittyAvatar";
import { TxButton } from "./substrate-lib/components";
import { useSubstrateState } from "./substrate-lib";

// --- About Modal ---

const TransferModal = (props) => {
  const { kitty, accountPair, setStatus } = props;
  const [open, setOpen] = useState(false);
  const [formValue, setFormValue] = useState({});

  const { keyring } = useSubstrateState();

  const options = keyring.getPairs().map(account => ({
    key: account.address,
    value: account.address,
    text: account.meta.name.toUpperCase(),
    icon: 'user',
  }))

  const formChange = (key) => (ev, el) => {
    console.log(el.value)
    setFormValue({ ...formValue, [key]: el.value });
  };

  const confirmAndClose = (unsub) => {
    unsub();
    setOpen(false);
  };

  return (
    <Modal
      onClose={() => setOpen(false)}
      onOpen={() => setOpen(true)}
      open={open}
      trigger={
        <Button basic color="blue">
          转让
        </Button>
      }
    >
      <Modal.Header>毛孩转让</Modal.Header>
      <Modal.Content>
        <Form>
          <Form.Input fluid label="毛孩 ID" readOnly value={kitty.id} />
          <Dropdown
            search
            selection
            placeholder="选择对方地址"
            options={options}
            onChange={formChange("target")}
          />
        </Form>
      </Modal.Content>
      <Modal.Actions>
        <Button basic color="grey" onClick={() => setOpen(false)}>
          取消
        </Button>
        <TxButton
          accountPair={accountPair}
          label="确认转让"
          type="SIGNED-TX"
          setStatus={setStatus}
          onClick={confirmAndClose}
          attrs={{
            palletRpc: "kittiesModule",
            callable: "transfer",
            inputParams: [kitty.id, formValue.target],
            paramFields: [true, true],
          }}
        />
      </Modal.Actions>
    </Modal>
  );
};

// --- About Kitty Card ---

const KittyCard = (props) => {
  const { kitty, accountPair, setStatus } = props;
  const { id, dna, owner } = kitty;

  const displayDna = (dna ?? []).join(", ");
  const displayId = `${id ?? ""}`.padStart(2, "0");
  const isSelf = accountPair.address === owner;

  return (
    <Card>
      {isSelf && (
        <Label as="a" floating color="teal">
          我的
        </Label>
      )}
      <KittyAvatar dna={dna} />
      <Card.Content>
        <Card.Header>ID 号: {displayId}</Card.Header>
        <Card.Meta style={{ overflowWrap: "break-word" }}>
          基因: <br />
          {displayDna}
        </Card.Meta>
        <Card.Description>
          <p style={{ overflowWrap: "break-word" }}>
            猫奴:
            <br />
            {owner}
          </p>
        </Card.Description>
      </Card.Content>
      <Card.Content extra style={{ textAlign: "center" }}>
        {isSelf && (
          <TransferModal
            kitty={kitty}
            accountPair={accountPair}
            setStatus={setStatus}
          />
        )}
      </Card.Content>
    </Card>
  );
};

const KittyCards = (props) => {
  const { kitties, accountPair, setStatus } = props;

  if (kitties.length === 0) {
    return (
      <Message info>
        <Message.Header>
          现在连一只毛孩都木有，赶快创建一只&nbsp;
          <span role="img" aria-label="point-down">
            👇
          </span>
        </Message.Header>
      </Message>
    );
  }

  return (
    <Grid columns={3}>
      {kitties.map((kitty, i) => (
        <Grid.Column key={`kitty-${i}`}>
          <KittyCard
            kitty={kitty}
            accountPair={accountPair}
            setStatus={setStatus}
          />
        </Grid.Column>
      ))}
    </Grid>
  );
};

export default KittyCards;
