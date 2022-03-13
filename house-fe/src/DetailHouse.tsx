import { Button, Slider, TextField } from "@material-ui/core"
import { useState } from "react"
import { useMutation, useQuery } from "react-query"
import { Link, useParams } from "react-router-dom"
import { HouseDTO } from "./List"

export default function DetailHouse() {
    const { houseId } = useParams()

    const [currVote, setVote] = useState<number | undefined>(undefined)
    const [currComment, setComment] = useState<string | undefined>(undefined)

    const houseQuery = useQuery<HouseDTO>(['houses', houseId], () => {
        return fetch('/api/houses/' + houseId).then(res => res.json())
    }, {
        refetchOnWindowFocus: false,
        refetchInterval: false,
        retry: false,
    })

    const updateHouseMutation = useMutation((body: UpdateHouseDto) => {
        return fetch('/api/houses/' + houseId, {
            method: 'PATCH',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify(body)
        })
    }, {
        onSuccess: () => houseQuery.refetch()
    })

    function update() {
        updateHouseMutation.mutate({
            comment: currComment,
            vote: currVote,
        })
    }

    if (houseQuery.isLoading) return (<div>Loading...</div>);
    if (houseQuery.error) return (<div>An error has occurred: </div>)
    if (!houseQuery.data) return (<div>OOOPS</div>)

    const {
        city,
        comment,
        link,
        vote,
        rooms_number,
        square_meters,
        street,
        zone,
        id,
    } = houseQuery.data

    return (
        <>
            <Link to={'/map?houseId=' + id}>Go to map</Link>
            <dl>
                <dt>Street</dt>
                <dd>{street}</dd>
                <dt>Zone</dt>
                <dd>{zone}</dd>
                <dt>City</dt>
                <dd>{city}</dd>
                <dt>Room</dt>
                <dd>{rooms_number}</dd>
                <dt>Square meters</dt>
                <dd>{square_meters}</dd>
                <dt>Link</dt>
                <dd><a target={'_blank'} href={link}>{link}</a></dd>
                <dt>Vote</dt>
                <dd>
                    <Slider
                        aria-labelledby="vote"
                        step={1}
                        marks
                        min={0}
                        max={10}
                        valueLabelDisplay="on"
                        onChange={(e, value) => setVote(value as number)}
                        value={currVote || vote}
                    />
                </dd>
                <dt>Comment</dt>
                <dd>
                    <TextField
                        id="comment"
                        label="Comment"
                        multiline
                        fullWidth
                        maxRows={4}
                        value={currComment || comment || ""}
                        onChange={e => setComment(e.target.value)}
                    />
                </dd>
            </dl>
            <Button variant="contained" color="primary" fullWidth onClick={update}>Update</Button>
        </>
    )
}

interface UpdateHouseDto {
    comment?: string,
    vote?: number,
}