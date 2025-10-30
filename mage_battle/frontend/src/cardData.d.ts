declare module './cardData.json' {
  interface Spell {
    spell_id: string;
    name: string;
    cost: string;
    effect: string;
  }

  interface Card {
    id: number;
    top_spell: Spell;
    bottom_spell: Spell | null;
  }

  const cardData: Card[];
  export default cardData;
}
