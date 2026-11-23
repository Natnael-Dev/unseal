use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, TransferChecked};
use solana_sha256_hasher::hashv;

// Public development identity. Replace with the actual deployment address before deployment.
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgMQhgYwR5Xh");

#[program]
pub mod unseal {
    use super::*;

    pub fn create_campaign(ctx: Context<CreateCampaign>, terms: Terms) -> Result<()> {
        terms.validate(Clock::get()?.unix_timestamp)?;
        require!(
            ctx.accounts.mint.freeze_authority.is_none(),
            UnsealError::FreezableMint
        );
        let campaign = &mut ctx.accounts.campaign;
        campaign.version = 1;
        campaign.bump = ctx.bumps.campaign;
        campaign.creator = ctx.accounts.creator.key();
        campaign.mint = ctx.accounts.mint.key();
        campaign.terms = terms;
        campaign.state = CampaignState::Funding;
        Ok(())
    }

    pub fn pledge(ctx: Context<Pledge>, amount: u64) -> Result<()> {
        let campaign = &mut ctx.accounts.campaign;
        let next = campaign.check_pledge(amount, Clock::get()?.unix_timestamp)?;
        let receipt = &mut ctx.accounts.receipt;
        // A receipt is never closed, so init_if_needed cannot reset a completed refund.
        if receipt.campaign == Pubkey::default() {
            receipt.campaign = campaign.key();
            receipt.sponsor = ctx.accounts.sponsor.key();
        }
        require!(!receipt.refunded, UnsealError::AlreadyRefunded);
        let contribution = receipt
            .amount
            .checked_add(amount)
            .ok_or(UnsealError::Overflow)?;
        token::transfer_checked(
            CpiContext::new(
                ctx.accounts.token_program.key(),
                TransferChecked {
                    from: ctx.accounts.source.to_account_info(),
                    mint: ctx.accounts.mint.to_account_info(),
                    to: ctx.accounts.vault.to_account_info(),
                    authority: ctx.accounts.sponsor.to_account_info(),
                },
            ),
            amount,
            ctx.accounts.mint.decimals,
        )?;
        receipt.amount = contribution;
        campaign.pledged = next;
        if next == campaign.terms.target {
            campaign.state = CampaignState::Filled;
        }
        Ok(())
    }

    pub fn release(ctx: Context<Release>, material: [u8; 32]) -> Result<()> {
        let campaign = &ctx.accounts.campaign;
        campaign.check_release(Clock::get()?.unix_timestamp)?;
        let campaign_key = campaign.key();
        let digest = hashv(&[
            b"unseal/reveal-v1",
            crate::ID.as_ref(),
            campaign_key.as_ref(),
            &material,
        ])
        .to_bytes();
        require!(
            digest == campaign.terms.commitment,
            UnsealError::WrongMaterial
        );
        let nonce = campaign.terms.nonce.to_le_bytes();
        let bump = [campaign.bump];
        let seeds: &[&[u8]] = &[b"campaign", campaign.creator.as_ref(), &nonce, &bump];
        token::transfer_checked(
            CpiContext::new(
                ctx.accounts.token_program.key(),
                TransferChecked {
                    from: ctx.accounts.vault.to_account_info(),
                    mint: ctx.accounts.mint.to_account_info(),
                    to: ctx.accounts.destination.to_account_info(),
                    authority: campaign.to_account_info(),
                },
            )
            .with_signer(&[seeds]),
            campaign.terms.target,
            ctx.accounts.mint.decimals,
        )?;
        let campaign = &mut ctx.accounts.campaign;
        campaign.material = material;
        campaign.state = CampaignState::Released;
        Ok(())
    }

    pub fn refund(ctx: Context<Refund>) -> Result<()> {
        let campaign = &ctx.accounts.campaign;
        campaign.check_refund(Clock::get()?.unix_timestamp)?;
        let receipt = &ctx.accounts.receipt;
        require!(
            !receipt.refunded && receipt.amount > 0,
            UnsealError::AlreadyRefunded
        );
        let total = campaign
            .refunded
            .checked_add(receipt.amount)
            .ok_or(UnsealError::Overflow)?;
        require!(total <= campaign.pledged, UnsealError::Overflow);
        let nonce = campaign.terms.nonce.to_le_bytes();
        let bump = [campaign.bump];
        let seeds: &[&[u8]] = &[b"campaign", campaign.creator.as_ref(), &nonce, &bump];
        token::transfer_checked(
            CpiContext::new(
                ctx.accounts.token_program.key(),
                TransferChecked {
                    from: ctx.accounts.vault.to_account_info(),
                    mint: ctx.accounts.mint.to_account_info(),
                    to: ctx.accounts.destination.to_account_info(),
                    authority: campaign.to_account_info(),
                },
            )
            .with_signer(&[seeds]),
            receipt.amount,
            ctx.accounts.mint.decimals,
        )?;
        ctx.accounts.receipt.refunded = true;
        let campaign = &mut ctx.accounts.campaign;
        campaign.refunded = total;
        campaign.state = CampaignState::Refunding;
        Ok(())
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub struct Terms {
    pub nonce: u64,
    pub target: u64,
    pub fund_by: i64,
    pub reveal_by: i64,
    pub manifest_digest: [u8; 32],
    pub ciphertext_digest: [u8; 32],
    pub commitment: [u8; 32],
}

impl Terms {
    fn validate(&self, now: i64) -> Result<()> {
        require!(self.target > 0, UnsealError::InvalidAmount);
        require!(
            now < self.fund_by && self.fund_by < self.reveal_by,
            UnsealError::Deadline
        );
        Ok(())
    }
}

#[account]
#[derive(InitSpace)]
pub struct Campaign {
    pub version: u8,
    pub bump: u8,
    pub creator: Pubkey,
    pub mint: Pubkey,
    pub terms: Terms,
    pub pledged: u64,
    pub refunded: u64,
    pub state: CampaignState,
    pub material: [u8; 32],
}

impl Campaign {
    fn check_pledge(&self, amount: u64, now: i64) -> Result<u64> {
        require!(
            self.state == CampaignState::Funding,
            UnsealError::WrongState
        );
        require!(now < self.terms.fund_by, UnsealError::Deadline);
        require!(amount > 0, UnsealError::InvalidAmount);
        let next = self
            .pledged
            .checked_add(amount)
            .ok_or(UnsealError::Overflow)?;
        require!(next <= self.terms.target, UnsealError::Overfunded);
        Ok(next)
    }
    fn check_release(&self, now: i64) -> Result<()> {
        require!(self.state == CampaignState::Filled, UnsealError::WrongState);
        require!(now < self.terms.reveal_by, UnsealError::Deadline);
        Ok(())
    }
    fn check_refund(&self, now: i64) -> Result<()> {
        let allowed = match self.state {
            CampaignState::Funding => now >= self.terms.fund_by,
            CampaignState::Filled => now >= self.terms.reveal_by,
            CampaignState::Refunding => true,
            CampaignState::Released => false,
        };
        require!(allowed, UnsealError::RefundUnavailable);
        Ok(())
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, InitSpace)]
pub enum CampaignState {
    Funding,
    Filled,
    Released,
    Refunding,
}

#[account]
#[derive(InitSpace)]
pub struct Receipt {
    pub campaign: Pubkey,
    pub sponsor: Pubkey,
    pub amount: u64,
    pub refunded: bool,
}

#[derive(Accounts)]
#[instruction(terms: Terms)]
pub struct CreateCampaign<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,
    pub mint: Account<'info, Mint>,
    #[account(init, payer = creator, space = 8 + Campaign::INIT_SPACE,
        seeds = [b"campaign", creator.key().as_ref(), &terms.nonce.to_le_bytes()], bump)]
    pub campaign: Account<'info, Campaign>,
    #[account(init, payer = creator, seeds = [b"vault", campaign.key().as_ref()], bump,
        token::mint = mint, token::authority = campaign)]
    pub vault: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Pledge<'info> {
    #[account(mut)]
    pub sponsor: Signer<'info>,
    #[account(mut, has_one = mint, seeds = [b"campaign", campaign.creator.as_ref(), &campaign.terms.nonce.to_le_bytes()], bump = campaign.bump)]
    pub campaign: Account<'info, Campaign>,
    pub mint: Account<'info, Mint>,
    #[account(init_if_needed, payer = sponsor, space = 8 + Receipt::INIT_SPACE,
        seeds = [b"pledge", campaign.key().as_ref(), sponsor.key().as_ref()], bump)]
    pub receipt: Account<'info, Receipt>,
    #[account(mut, token::mint = mint, token::authority = sponsor)]
    pub source: Account<'info, TokenAccount>,
    #[account(mut, seeds = [b"vault", campaign.key().as_ref()], bump, token::mint = mint, token::authority = campaign)]
    pub vault: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Release<'info> {
    pub caller: Signer<'info>,
    #[account(mut, has_one = mint, seeds = [b"campaign", campaign.creator.as_ref(), &campaign.terms.nonce.to_le_bytes()], bump = campaign.bump)]
    pub campaign: Account<'info, Campaign>,
    pub mint: Account<'info, Mint>,
    #[account(mut, seeds = [b"vault", campaign.key().as_ref()], bump, token::mint = mint, token::authority = campaign)]
    pub vault: Account<'info, TokenAccount>,
    #[account(mut, token::mint = mint, token::authority = campaign.creator)]
    pub destination: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct Refund<'info> {
    pub caller: Signer<'info>,
    #[account(mut, has_one = mint, seeds = [b"campaign", campaign.creator.as_ref(), &campaign.terms.nonce.to_le_bytes()], bump = campaign.bump)]
    pub campaign: Account<'info, Campaign>,
    pub mint: Account<'info, Mint>,
    #[account(mut, has_one = campaign, seeds = [b"pledge", campaign.key().as_ref(), receipt.sponsor.as_ref()], bump)]
    pub receipt: Account<'info, Receipt>,
    #[account(mut, seeds = [b"vault", campaign.key().as_ref()], bump, token::mint = mint, token::authority = campaign)]
    pub vault: Account<'info, TokenAccount>,
    #[account(mut, token::mint = mint, token::authority = receipt.sponsor)]
    pub destination: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

#[error_code]
pub enum UnsealError {
    #[msg("The campaign is in the wrong state")]
    WrongState,
    #[msg("The instruction is outside its time window")]
    Deadline,
    #[msg("The amount must be positive")]
    InvalidAmount,
    #[msg("The amount exceeds remaining funding")]
    Overfunded,
    #[msg("Arithmetic exceeds recorded liabilities")]
    Overflow,
    #[msg("This contribution has already been refunded")]
    AlreadyRefunded,
    #[msg("Refunds are unavailable in this state or time window")]
    RefundUnavailable,
    #[msg("The release material does not match the commitment")]
    WrongMaterial,
    #[msg("The campaign mint must not have a freeze authority")]
    FreezableMint,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn campaign() -> Campaign {
        Campaign {
            version: 1,
            bump: 1,
            creator: Pubkey::new_unique(),
            mint: Pubkey::new_unique(),
            terms: Terms {
                nonce: 1,
                target: 5,
                fund_by: 10,
                reveal_by: 20,
                manifest_digest: [1; 32],
                ciphertext_digest: [2; 32],
                commitment: [3; 32],
            },
            pledged: 0,
            refunded: 0,
            state: CampaignState::Funding,
            material: [0; 32],
        }
    }

    #[test]
    fn pledge_cap_and_exact_funding_boundary() {
        let mut c = campaign();
        assert!(c.check_pledge(0, 1).is_err());
        assert!(c.check_pledge(6, 1).is_err());
        c.pledged = c.check_pledge(2, 1).unwrap();
        assert_eq!(c.check_pledge(3, 9).unwrap(), 5);
        assert!(c.check_pledge(3, 10).is_err());
        c.pledged = u64::MAX;
        assert!(c.check_pledge(1, 1).is_err());
    }

    #[test]
    fn release_and_refund_windows_do_not_overlap() {
        let mut c = campaign();
        assert!(c.check_release(1).is_err());
        assert!(c.check_refund(9).is_err());
        assert!(c.check_refund(10).is_ok());
        c.state = CampaignState::Filled;
        assert!(c.check_release(19).is_ok());
        assert!(c.check_refund(19).is_err());
        assert!(c.check_release(20).is_err());
        assert!(c.check_refund(20).is_ok());
        c.state = CampaignState::Released;
        assert!(c.check_release(21).is_err());
        assert!(c.check_refund(21).is_err());
        assert!(c.check_pledge(1, 1).is_err());
        c.state = CampaignState::Refunding;
        assert!(c.check_refund(21).is_ok());
        assert!(c.check_release(1).is_err());
        assert!(c.check_pledge(1, 1).is_err());
    }

    #[test]
    fn immutable_terms_require_ordered_deadlines_and_positive_target() {
        let mut t = campaign().terms;
        assert!(t.validate(1).is_ok());
        assert!(t.validate(10).is_err());
        t.reveal_by = 10;
        assert!(t.validate(1).is_err());
        t.reveal_by = 20;
        t.target = 0;
        assert!(t.validate(1).is_err());
    }

    #[test]
    fn serialized_accounts_fit_allocated_space() {
        let mut bytes = Vec::new();
        campaign().try_serialize(&mut bytes).unwrap();
        assert_eq!(bytes.len(), 8 + Campaign::INIT_SPACE);
        bytes.clear();
        Receipt {
            campaign: Pubkey::new_unique(),
            sponsor: Pubkey::new_unique(),
            amount: 5,
            refunded: false,
        }
        .try_serialize(&mut bytes)
        .unwrap();
        assert_eq!(bytes.len(), 8 + Receipt::INIT_SPACE);
    }
}
