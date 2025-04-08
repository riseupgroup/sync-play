<script lang="ts">
    import { onMount } from "svelte";
    import {
        Heading,
        A,
        Button,
        Modal,
        Table,
        TableBody,
        TableBodyCell,
        TableBodyRow,
        TableHead,
        TableHeadCell,
        FloatingLabelInput
    } from "flowbite-svelte";
    import { MouseClick, Room } from "../../../app";

    let createRoomOpen: boolean = false;
    let newRoomName: string = "";
    let rooms: Room[] = [];

    onMount(() => {
        getWebsocketSessions();
    });

    async function getWebsocketSessions() {
        let res = await fetch("/api/rooms");
        if (res.ok) {
            let _rooms = await res.json();
            for (let room in _rooms) {
                room = Object.assign(new Room(), room);
            }
            rooms = _rooms;
        } else {
            alert(res.text());
            rooms = [];
        }
    }

    async function newRoom() {
        fetch("/api/rooms", {
            method: "POST",
            headers: {
                "Content-Type": "application/json"
            },
            body: JSON.stringify({ name: newRoomName })
        }).then((res) => {
            if (res.ok) {
                getWebsocketSessions();
                newRoomName = "";
            } else {
                alert(res.text());
            }
        });
    }
</script>

<div>
    <Heading tag="h1" class="mb-4"
        >Websocket Sessions <A on:click={() => (createRoomOpen = true)}>+ New Room</A></Heading
    >
    <Table>
        <TableHead>
            <TableHeadCell>ID</TableHeadCell>
            <TableHeadCell>Name</TableHeadCell>
            <TableHeadCell>Members</TableHeadCell>
            <TableHeadCell>Actions</TableHeadCell>
        </TableHead>
        <TableBody>
            {#each rooms as room}
                <TableBodyRow>
                    <TableBodyCell>{room.id}</TableBodyCell>
                    <TableBodyCell>{room.name}</TableBodyCell>
                    <TableBodyCell>{room.members.length}</TableBodyCell>
                    <TableBodyCell
                        ><A on:click={(event) => new MouseClick(event).goto("/rooms/" + room.id)}
                            >Open</A
                        ></TableBodyCell
                    >
                </TableBodyRow>
            {/each}
        </TableBody>
    </Table>
    <Modal title="New Room" bind:open={createRoomOpen} size="xs" autoclose outsideclose>
        <FloatingLabelInput style="outlined" classLabel="cursor-text bg-white dark:bg-gray-800" classDiv="" bind:value={newRoomName}>Name</FloatingLabelInput>
        <svelte:fragment slot="footer">
            <Button color="alternative" class="ml-auto">Cancel</Button>
            <Button on:click={newRoom}>Create Room</Button>
        </svelte:fragment>
    </Modal>
</div>
