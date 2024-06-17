ALTER TABLE successful_end_to_end_charge ADD COLUMN public_id UUID UNIQUE NOT NULL DEFAULT uuid_generate_v4();
ALTER TABLE wallet_card_charge ADD COLUMN public_id UUID UNIQUE NOT NULL DEFAULT uuid_generate_v4();
