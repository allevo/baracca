import React from 'react';
import { findByText, fireEvent, getByRole, render, screen, waitFor, within } from '@testing-library/react';
import List from './List';
import { setupServer } from 'msw/node'

import { QueryClient, QueryClientProvider } from 'react-query';
import { rest } from 'msw';

const createTestQueryClient = () => new QueryClient({
  defaultOptions: {
    queries: {
      retry: false,
    },
  },
})

export function renderWithClient(ui: React.ReactElement) {
  const testQueryClient = createTestQueryClient()
  const { rerender, ...result } = render(
    <QueryClientProvider client={testQueryClient}>{ui}</QueryClientProvider>
  )
  return {
    ...result,
    rerender: (rerenderUi: React.ReactElement) =>
      rerender(
        <QueryClientProvider client={testQueryClient}>{rerenderUi}</QueryClientProvider>
      ),
  }
}

const server = setupServer(rest.get('*', (req, res, ctx) => {
  return res(
    ctx.status(500),
    ctx.json({
      error: req.url
    })
  )
}))

// Establish API mocking before all tests.
beforeAll(() => server.listen());
// Reset any request handlers that we may add during the tests,
// so they don't affect other tests.
afterEach(() => server.resetHandlers());
// Clean up after the tests are finished.
afterAll(() => server.close());

test('renders learn react link', async () => {
  let count = 0;
  server.use(
    rest.get('/api/houses', (req, res, ctx) => {
      if (count == 0)
        return res(
          ctx.status(200),
          ctx.json([
            { "id": "1", "link": "http://example.org/1/", "vote": 7, "comment": "", "city": "Milano", "zone": "Piazzale Siena", "street": "Via Tommaso Gulli, 32", "lat": 45.4654, "lng": 9.1334, "rooms_number": 2, "square_meters": 71 },
            { "id": "2", "link": "http://example.org/2/", "vote": 4, "comment": "", "city": "Milano", "zone": "Dergano", "street": "Via Pellegrino Rossi, 13", "lat": 45.5081, "lng": 9.1775, "rooms_number": 2, "square_meters": 60 }
          ])
        )
      else
        return res(
          ctx.status(200),
          ctx.json([
            { "id": "2", "link": "http://example.org/2/", "vote": 4, "comment": "", "city": "Milano", "zone": "Dergano", "street": "Via Pellegrino Rossi, 13", "lat": 45.5081, "lng": 9.1775, "rooms_number": 2, "square_meters": 60 }
          ])
        )
    }))


  const result = renderWithClient(<List />)

  const linkElement = result.getByText(/Loading/i)
  expect(linkElement).toBeInTheDocument()

  const text = await result.findByText(/Via Tommaso Gulli, 32 \(Piazzale Siena\)/i)
  expect(text).toBeInTheDocument()

  const first = result.getByTestId('item-list-1')
  expect(first).toBeInTheDocument()
  const firstDelete = within(first).getByLabelText('Remove element')
  expect(firstDelete).toBeInTheDocument()
  const second = result.getByTestId('item-list-2')
  expect(second).toBeInTheDocument()
  const secondDelete = within(second).getByLabelText('Remove element')
  expect(secondDelete).toBeInTheDocument()

  server.use(
    rest.delete('/api/houses/1', (req, res, ctx) => {
      return res(
        ctx.status(204),
        ctx.json(null)
      )
    }))

  fireEvent.click(firstDelete)

  await waitFor(async () => {
    expect(await screen.findByText(/Via Tommaso Gulli, 32 \(Piazzale Siena\)/i)).not.toBeInTheDocument()
  })
});
