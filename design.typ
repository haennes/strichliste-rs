#let box_width = 7em;
#let box_height = 4em;
#let userbox(count) = {
    box(inset: 10pt, fill: luma(200), height: box_height, width: box_width, [user #count \ money] )
  };
#let userbox_link_to_user(count) = link(<UserPage>, userbox(count))
#let itembox(n) = box(width: box_width, height: box_width, stroke: 2pt, inset:2pt, align(center+bottom, stack([IMG],[ITEM #n])));
#let moneybox = box(width: box_width, height: box_height, stroke: 2pt, align(center+horizon)[geld]);
#let user_count = 20;
#let top_bar = table(columns: (auto, auto, 1fr, auto), [Active Users], [Inactive Users], [], [Health]);


#let transaction(big: true, n) = {
  let amount = calc.round(calc.pow(-1.5,n), digits: 2);
  let time = if big {datetime(
      year: 2010,
      month: 10,
      day: 10,
      hour: 10,
      minute: 10,
      second: n
    )} else {
      datetime(hour: 10, minute: 10, second: n)
    };
  let amount_fmt = [
    #set text(stroke: 0.5pt + (if amount < 0 {red} else {green}))
    #amount €
  ];
  (time.display(), amount_fmt,[UNDO])
};

#let transaction_small(n) = transaction(big: false, n);
#let transaction_log(count) = table(columns: (auto, 1fr, auto), stroke:none, align: horizon, ..range(count).map(transaction).flatten());
#let transaction_log_small(count) = table(columns: (auto, 1fr, auto), stroke:none, align: horizon, ..range(count).map(transaction_small).flatten());

#let spacing = 10pt;
#let wide_arrow_up = scale(x: box_width, math.hat);
#let wide_arrow_down = scale(y: -1em, wide_arrow_up);


#set stack(spacing: spacing)
#set align(center)

= ItemSelector (indirekt genutzt) <ItemSelector>

#top_bar
#wide_arrow_up
#align(center, box(
  inset: 1em,
  grid(columns: 5, gutter: spacing, ..range(user_count).map(itembox))
))

#wide_arrow_down

= UserSelector (indirekt genutzt) <UserSelector>
#box(inset: 1em, grid(columns: 5, gutter: spacing, ..range(user_count).map(userbox)))


#pagebreak()
#heading[ MainPage  (hier ist ein #link(<UserSelector>)[UserSelector] embedded) <MainPage>]
\
#top_bar
#wide_arrow_up
#box(inset: 1em, grid(columns: 5, gutter: spacing, ..range(user_count).map(userbox_link_to_user)))
//#box(inset: 5%, (range(user_count).map(userbox).join()))
#wide_arrow_down

#pagebreak()
= Transfer <Transfer>

#box(height: 50%)[
#top_bar
#align(center+horizon,stack(
  stack(link(<ItemSelector>,itembox(1)), moneybox),
  stack(dir: ltr, link(<UserSelector>,userbox("from")),$->$ , link(<UserSelector>, userbox("to"))))
)
]

#pagebreak()

= Einzahlen / Auszahlen <CashInOut>

#top_bar
#table(columns: 2, align: horizon, inset: spacing,
[Einzahlen], text(size:40pt)[-])

#let coin_height = 40pt
#let coin(c) = circle(radius: coin_height/2)[
  #set align(center + horizon)
  #c€
];
#let bank_note(n) = box(height:coin_height, width: 2*coin_height, stroke: 1pt)[
  #set align(center + horizon)
  #n€
]
#stack(dir:ltr,
..(0.1, 0.5, 1, 2).map(coin),
..(10,20).map(bank_note)
)
#moneybox

#box(width: 80%, transaction_log(16))


#pagebreak()

= UserPage <UserPage>
#top_bar
#let item_rows_count = 3;
#align(left)[nickname]
#table(columns: (30%, 1fr, 1fr, 1fr, 30%),
  [\$GELD], link(<CashInOut>)[-], link(<Transfer>,sym.arrows.rl), link(<CashInOut>)[+], [Transaction Log]
)
#grid(columns: (66%, 33%), align: (left, right),
  {
    let columns = 3;
    grid(columns: columns, gutter: spacing, align: left,
       ..(range(item_rows_count*columns - 1).map(itembox))+
       (box(width: box_width, height: box_width, stroke: 2pt, inset:2pt,
         align(center+horizon, link(<ItemSelector>)[MORE])),
       )

    )
  },
  transaction_log_small(10)
)
