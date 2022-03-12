import { Button, createStyles, Grid, makeStyles, Slider, TextField, Theme, Typography } from "@material-ui/core";
import { useState } from "react";
import { useMutation, useQuery } from "react-query";


const useStyles = makeStyles((theme: Theme) =>
    createStyles({
        root: {
            width: '100%',
        },
    }),
);

export function InsertHouse() {
    const classes = useStyles();

    const [link, setLink] = useState('');
    const [vote, setVote] = useState(5);
    const [comment, setComment] = useState('');

    const insertHouseMutation = useMutation((body: InsertHouseDto) => {
        return fetch('/api/houses', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify(body)
        })
    })


    const discoverQuery = useQuery<DiscoverResultDTO, Error>(
        ['discover', link],
        async () => {
            const res = await fetch('/api/discover?url=' + link)

            if (res.status === 404) {
                throw new Error('Not found')
            }

            return res.json()
        },
        {
            refetchOnWindowFocus: false,
            enabled: false,
            refetchInterval: false,
            retry: false,
        }
    )

    if (insertHouseMutation.isLoading) {
        return <div>Inserting house...</div>
    }
    if (insertHouseMutation.isError) {
        return <div>Error</div>
    }
    if (insertHouseMutation.isSuccess) {
        return <div>Done</div>
    }

    function insert() {
        insertHouseMutation.mutate({ link, vote, comment, ...discoverQuery.data })
    }
    function fetchInfo() {
        discoverQuery.refetch()
    }
    function onChangeLink(e: React.ChangeEvent<HTMLInputElement>) {
        setLink(e.target.value)
        discoverQuery.remove()
    }

    const discoverError = discoverQuery.error && <p style={{ color: 'red' }}>{discoverQuery.error.message}</p>
    const isDiscovered = !!discoverQuery.data

    return (
        <>
            <TextField id="link" label="Link" fullWidth value={link} onChange={onChangeLink} />
            <Button onClick={fetchInfo}>fetchInfo</Button>
            {discoverError}
            {isDiscovered && <DiscoveryResult data={discoverQuery.data} />}
            <div>
                <Typography id="vote" gutterBottom>Vote</Typography>
                <Slider
                    aria-labelledby="vote"
                    step={1}
                    marks
                    min={0}
                    max={10}
                    valueLabelDisplay="on"
                    onChange={(e, value) => setVote(value as number)}
                    value={vote}
                />
            </div>
            <TextField
                id="comment"
                label="Comment"
                multiline
                fullWidth
                maxRows={4}
                value={comment}
                onChange={e => setComment(e.target.value)}
            />

            <Button variant="contained" color="primary" fullWidth onClick={insert}>Insert</Button>
        </>
    )
}

function DiscoveryResult({ data }: { data: DiscoverResultDTO }) {
    return (<div>
        <p>{data.city} {data.street} ({data.zone})</p>
        <p>{data.rooms_number} locali, {data.square_meters}mq</p>
    </div>
    )
}

export interface InsertHouseDto extends DiscoverResultDTO {
    link: string,
    vote: number,
    comment: string,
}

interface DiscoverResultDTO {
    city?: string,
    lat?: number,
    lng?: number,
    rooms_number?: number,
    square_meters?: number,
    street?: string,
    zone?: string,
}